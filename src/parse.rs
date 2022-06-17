use mlua::Lua;
use mlua::Error;

use std::io::Cursor;
use std::io::Read;

use super::instr::Function;
use super::instr::size_t;

#[derive(Debug)]
pub enum ParserError {
    LuaError(Error),
    NoBytesLeft(usize, i32),
    NotLua,
    WrongVersion
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

/// Uses the provided lua state and bytecode in order to
/// properly read a lua String from the bytecode.
fn consume_string<'lua>(bytecode: &mut Cursor<Vec<u8>>, state: &'lua Lua) -> Result<mlua::String<'lua>, ParserError> {
    let len: size_t = consume!(bytecode, 4)?[0] as size_t;
    let res = state.create_string(&consume_vec!(bytecode, len as usize)?)
        .map_err(|e| ParserError::LuaError(e))?;

    return Ok(res);
}

/// Take the original lua source code, and compile it into
/// Lua 5.1 bytecode.
fn to_bytecode(src: &String, state: &Lua) -> Result<Cursor<Vec<u8>>, Error> {
    let func = state.load(src).into_function()?;
    return Ok(Cursor::new(func.dump(true)));
}

/// Deserialize a function block.
fn deserialize_function(bytecode: &mut Cursor<Vec<u8>>, little_endian: bool, state: &Lua) -> Result<Function, ParserError> {
    // Source name
    let source_name = consume_string(bytecode, &state)?;

    println!("{:?}", source_name.to_string_lossy());

    todo!()
}


/// Deserialize a chunk of lua bytecode
pub fn deserialize(src: &String) -> Result<Function, ParserError> {
    let state = Lua::new(); // For lua type generation, and compilation
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
    return deserialize_function(&mut bytecode, is_little_endian, &state);
}