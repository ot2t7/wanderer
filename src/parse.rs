use mlua::Lua;
use mlua::Error;
use strum::IntoEnumIterator;

use std::io::Cursor;
use std::io::Read;

use crate::instr::InstructionKind;
use crate::instr::Local;
use crate::instr::OpCode;
use crate::instr::Instruction;
use super::instr::Function;
use super::instr::SizeT;
use super::instr::Vararg;
use super::instr::Integer;
use super::instr::Constant;
use super::instr::Number;
use super::instr::INSTRUCTION_MAP;
use super::instr::make_instruction;

#[derive(Debug)]
pub enum ParserError {
    LuaError(Error),
    NoBytesLeft(usize, i32),
    NotLua,
    WrongVersion,
    UnknownInstruction
}

/// Go to a cursor and pop off a certain amount of bytes off
/// at the front, into a buffer. The nice thing about using a
/// macro in this situation is that the buffer is held entirely
/// on the stack, which means the allocation is pretty quick.
/// Returns a `ParserError::NoBytesLeft` if not enough bytes can
/// be read.
#[macro_export]
macro_rules! consume {
    ($reader: expr, $size: literal) => {
        {
            (||{
                let mut buf: [u8 ; $size] = [0 ; $size];
                match $reader.read(&mut buf) {
                    Ok(num_bytes_read) => {
                        if num_bytes_read < $size {
                            return Err(ParserError::NoBytesLeft($size, num_bytes_read as i32));
                        } else {
                            return Ok(buf);
                        }
                    }
                    Err(_) => { return Err(ParserError::NoBytesLeft($size, -1)); }
                }
            })()
        }
    };
}

/// Works in the same way as `consume!`, but it allows reading
/// from the reader an non constant amount of bytes. This will
/// be slower since it allocates byte by byte and allocates
/// to a vector instead of a sized buffer.
/// Also returns a `ParserError::NoBytesLeft` if not enough 
/// bytes are left to be read.
#[macro_export]
macro_rules! consume_vec {
    ($reader: expr, $size: expr) => {
        {
            (||{
                let mut buf: Vec<u8> = vec![];
                for _ in 0..$size {
                    buf.push(consume!($reader, 1)?[0]);
                }
                return Ok(buf);
            })()
        }
    };
}

/// Uses the provided lua state in order to properly
/// read a lua SizeT from the bytecode.
fn consume_size_t(bytecode: &mut Cursor<Vec<u8>>) -> Result<SizeT, ParserError> {
    return Ok(SizeT::from_le_bytes(consume!(bytecode, 8)?));
}

/// Uses the provided lua state and bytecode in order to
/// properly read a lua String from the bytecode.
fn consume_string<'lua>(bytecode: &mut Cursor<Vec<u8>>, state: &'lua Lua) -> Result<mlua::String<'lua>, ParserError> {
    let mut len: SizeT = consume_size_t(bytecode)?;
    let mut null_terminated = false;
    if len > 0 { len = len - 1; null_terminated = true; } // Lua strings have a null terminator at the end, we don't wanna read that
    let res = state.create_string(&consume_vec!(bytecode, len)?)
        .map_err(|e| ParserError::LuaError(e))?;

    if null_terminated == true { consume!(bytecode, 1)?; } // We skipped a byte, let's account for that
    return Ok(res);
}

/// Uses the provided bytecode in order to properly
/// read a lua Integer from the bytecode.
fn consume_integer(bytecode: &mut Cursor<Vec<u8>>) -> Result<Integer, ParserError> {
    return Ok(i32::from_le_bytes(consume!(bytecode, 4)?));
}

/// Uses the provided bytecode in order to properly
/// read a lua number/float from the bytecode.
fn consume_number(bytecode: &mut Cursor<Vec<u8>>) -> Result<Number, ParserError> {
    return Ok(Number::from_le_bytes(consume!(bytecode, 8)?));
}

/// Take in a value which represents the ID of an 
/// opcode, and then spit out an OpCode enum.
fn index_instruction(opcode: u8) -> Result<OpCode, ParserError> {
    let instr = OpCode::iter().nth(opcode as usize);
    match instr {
        Some(i) => return Ok(i),
        None => Err(ParserError::UnknownInstruction)
    }
}

/// Uses the provided bytecode in order to properly
/// read a lua instruction from the bytecode. Some
/// bitwise operations have been taken from the 
/// Ironbrew 2 source code.
/// https://github.com/Trollicus/ironbrew-2
fn consume_instruction(bytecode: &mut Cursor<Vec<u8>>) -> Result<Instruction, ParserError> {
    let data = i32::from_le_bytes(consume!(bytecode, 4)?);
    let opcode_num = data & 0x3F;
    let op_code = index_instruction(opcode_num as u8)?;
    let instruction_kind = INSTRUCTION_MAP.get(&op_code).cloned().unwrap();
    let mut template = make_instruction(op_code, instruction_kind);
    template.a = (data >> 6) & 0xFF;
    match template.instruction_kind {
        InstructionKind::ABC => {
            template.b = Some((data >> 6 + 8 + 9) & 0x1FF);
            template.c = Some((data >> 6 + 8) & 0x1FF);
        },
        InstructionKind::ABx => {
            template.bx = Some((data >> 6 + 8) & 0x3FFFF);
        },
        InstructionKind::AsBx => {
            template.sbx = Some(((data >> 6 + 8) & 0x3FFFF) - 131071);
        }
    }

    return Ok(template);
}

/// Take the original lua source code, and compile it into
/// Lua 5.1 bytecode.
fn to_bytecode(src: &String, state: &Lua) -> Result<Cursor<Vec<u8>>, Error> {
    let func = state.load(src).into_function()?;
    std::fs::write("out", func.dump(false)).unwrap();
    return Ok(Cursor::new(func.dump(false)));
}

/// Deserialize a function block.
fn deserialize_function<'a>(bytecode: &mut Cursor<Vec<u8>>, little_endian: bool, state: &'a Lua) -> Result<Function<'a>, ParserError> {
    // Source name
    let source_name = consume_string(bytecode, &state)?;
    // Line defined, and last line defined
    let line_defined = consume_integer(bytecode)?;
    let last_line_defined = consume_integer(bytecode)?;
    // Number of upvalues
    let num_upvalues = consume!(bytecode, 1)?[0];
    // Number of parameters
    let num_parameters = consume!(bytecode, 1)?[0];
    // is_vararg
    let is_vararg;
    match consume!(bytecode, 1)?[0] {
        1 => is_vararg = Vararg::HasArg,
        2 => is_vararg = Vararg::IsVararg,
        4 => is_vararg = Vararg::NeedsVararg,
        _ => is_vararg = Vararg::HasArg // Idk lets just assume 1
    }
    // Maximum stack size, or the number of registers used
    let stack_size = consume!(bytecode, 1)?[0];

    // Instruction list
    let sizecode = consume_integer(bytecode)?;
    let mut instructions = Vec::with_capacity(sizecode as usize);
    for _ in 0..sizecode {
        instructions.push(consume_instruction(bytecode)?);
    }
    println!("instructions: {:?}", instructions);

    // Constant list
    let sizek = consume_integer(bytecode)?;
    let mut constants = Vec::with_capacity(sizek as usize);
    for _ in 0..sizek {
        let constant_type;
        match consume!(bytecode, 1)?[0] {
            0 => constant_type = Constant::Nil,
            1 => constant_type = Constant::Boolean(consume!(bytecode, 1)?[0] == 1),
            3 => constant_type = Constant::Number(consume_number(bytecode)?),
            4 => constant_type = Constant::String(consume_string(bytecode, state)?),
            _ => constant_type = Constant::Nil // Assume nil
        }
        constants.push(constant_type);
    }

    // Function prototype list
    let sizep = consume_integer(bytecode)?;
    let mut protos = Vec::with_capacity(sizek as usize);
    for _ in 0..sizep {
        protos.push(deserialize_function(bytecode, little_endian, state)?);
    }

    // Source line position list (debug)
    let sizelineinfo = consume_integer(bytecode)?;
    // Index of this Vec represents the instruction position, val represents line number in src
    let mut instr_positions = Vec::with_capacity(sizelineinfo as usize);
    for _ in 0..sizelineinfo {
        instr_positions.push(consume_integer(bytecode)?);
    }

    // Local list (debug)
    let sizelocvars = consume_integer(bytecode)?;
    let mut locals = Vec::with_capacity(sizelocvars as usize);
    for _ in 0..sizelocvars {
        let var_name = consume_string(bytecode, state)?;
        let start_pc = consume_integer(bytecode)?;
        let end_pc = consume_integer(bytecode)?;
        locals.push(Local {
            var_name,
            start_pc,
            end_pc
        })
    }

    // Upvalue list (debug)
    let sizeupvalues = consume_integer(bytecode)?;
    let mut upvalues = Vec::with_capacity(sizeupvalues as usize);
    for _ in 0..sizeupvalues {
        upvalues.push(consume_string(bytecode, state)?);
    }

    println!("lines: {:?}", instr_positions);
    println!("locals: {:?}", locals);
    println!("upvalues: {:?}", upvalues);

    return Ok(Function {
        source_name,
        line_defined,
        last_line_defined,
        num_upvalues,
        num_parameters,
        is_vararg,
        stack_size,
        instructions,
        constants, 
        function_protos: protos,
        instruction_positions: instr_positions,
        name_locals: locals,
        name_upvalues: upvalues
    });
}


/// Deserialize a chunk of lua bytecode
pub fn deserialize<'a>(src: &String, state: &'a Lua) -> Result<Function<'a>, ParserError> {
    let mut bytecode = to_bytecode(src, &state)
        .map_err(|e| ParserError::LuaError(e))?;
    
    // Read the lua header block

    // Header Signature, should be 0x1B4C7561
    if u32::from_be_bytes(consume!(bytecode, 4)?) != 457995617 {
        return Err(ParserError::NotLua);
    }
    // Version number, should be 0x51 for Lua 5.1
    if consume!(bytecode, 1)?[0] != 81 {
        return Err(ParserError::WrongVersion);
    }
    // Format version, we skip this
    consume!(bytecode, 1)?;
    // Endianness flag, 0=big endian, 1=little endian
    let is_little_endian = consume!(bytecode, 1)?[0] == 1;
    // Size of int, size_t, Instruction, and lua_Number, and the integral flag, we skip this
    consume!(bytecode, 5)?;
    // Begin eating up the function blocks
    return deserialize_function(&mut bytecode, is_little_endian, state);
}