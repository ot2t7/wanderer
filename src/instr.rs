use std::os::raw::c_double;
use std::collections::HashMap;

use strum_macros::EnumIter;
use lazy_static::lazy_static;

/// Every single Lua 5.1 OpCode
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OpCode {
    Move,
    Loadk, 
    LoadBool, 
    LoadNil, 
    GetUpval, 
    GetGlobal, 
    GetTable, 
    SetGlobal, 
    SetUpval, 
    SetTable, 
    NewTable, 
    _Self, // Self is already reserved by rust
    Add, 
    Sub, 
    Mul, 
    Div, 
    Mod, 
    Pow, 
    Unm, 
    Not, 
    Len, 
    Concat, 
    Jmp, 
    Eq, 
    Lt, 
    Le, 
    Test, 
    TestSet, 
    Call, 
    TailCall, 
    Return, 
    ForLoop, 
    ForPrep, 
    TForLoop, 
    SetList, 
    Close, 
    Closure, 
    Vararg
}

lazy_static! {
    /// A hashmap that allows a lookup of OpCode's in order to
    /// get their proper instruction type.
    pub static ref INSTRUCTION_MAP: HashMap<OpCode, InstructionKind> = HashMap::from([
        ( OpCode::Move, InstructionKind::ABC ),
        ( OpCode::Loadk, InstructionKind::ABx ),
        ( OpCode::LoadBool, InstructionKind::ABC ),
        ( OpCode::LoadNil, InstructionKind::ABC ),
        ( OpCode::GetUpval, InstructionKind::ABC ),
        ( OpCode::GetGlobal, InstructionKind::ABx ),
        ( OpCode::GetTable, InstructionKind::ABC ),
        ( OpCode::SetGlobal, InstructionKind::ABx ),
        ( OpCode::SetUpval, InstructionKind::ABC ),
        ( OpCode::SetTable, InstructionKind::ABC ),
        ( OpCode::NewTable, InstructionKind::ABC ),
        ( OpCode::_Self, InstructionKind::ABC ),
        ( OpCode::Add, InstructionKind::ABC ),
        ( OpCode::Sub, InstructionKind::ABC ),
        ( OpCode::Mul, InstructionKind::ABC ),
        ( OpCode::Div, InstructionKind::ABC ),
        ( OpCode::Mod, InstructionKind::ABC ),
        ( OpCode::Pow, InstructionKind::ABC ),
        ( OpCode::Unm, InstructionKind::ABC ),
        ( OpCode::Not, InstructionKind::ABC ),
        ( OpCode::Len, InstructionKind::ABC ),
        ( OpCode::Concat, InstructionKind::ABC ),
        ( OpCode::Jmp, InstructionKind::AsBx ),
        ( OpCode::Eq, InstructionKind::ABC ),
        ( OpCode::Lt, InstructionKind::ABC ),
        ( OpCode::Le, InstructionKind::ABC ),
        ( OpCode::Test, InstructionKind::ABC ),
        ( OpCode::TestSet, InstructionKind::ABC ),
        ( OpCode::Call, InstructionKind::ABC ),
        ( OpCode::TailCall, InstructionKind::ABC ),
        ( OpCode::Return, InstructionKind::ABC ),
        ( OpCode::ForLoop, InstructionKind::AsBx ),
        ( OpCode::ForPrep, InstructionKind::AsBx ),
        ( OpCode::TForLoop, InstructionKind::ABC ),
        ( OpCode::SetList, InstructionKind::ABC ),
        ( OpCode::Close, InstructionKind::ABC ),
        ( OpCode::Closure, InstructionKind::ABx ),
        ( OpCode::Vararg, InstructionKind::ABC )
    ]);
}

/// ABC, ABx, or AsBx.
#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    ABC,
    ABx,
    AsBx
}

/// A deserialized instruction which can represent any kind of 
/// instruction, including ABC, ABx and AsBx. The size of
/// fields may not represent the actual number of bits on the
/// serialized instruction.
#[derive(Debug)]
pub struct Instruction {
    pub op_code: OpCode,
    pub instruction_kind: InstructionKind,
    pub a: i32,
    pub c: Option<i32>,
    pub b: Option<i32>,
    pub bx: Option<i32>,
    pub sbx: Option<i32>
}

/// Makes an empty instruction with all default values
pub fn make_instruction(op_code: OpCode, instruction_kind: InstructionKind) -> Instruction {
    match instruction_kind {
        InstructionKind::ABC => return Instruction {
            op_code, 
            instruction_kind, 
            a: 0, 
            b: Some(0), 
            c: Some(0),
            bx: None,
            sbx: None
        },
        InstructionKind::ABx => return Instruction {
            op_code, 
            instruction_kind, 
            a: 0, 
            b: None, 
            c: None,
            bx: Some(0),
            sbx: None
        },
        InstructionKind::AsBx => return Instruction {
            op_code, 
            instruction_kind, 
            a: 0, 
            b: None, 
            c: None,
            bx: None,
            sbx: Some(0)
        }
    }
}

/// An instance of this struct represents a deserialized function block
/// that has been extracted from the inputted bytecode. Most fields are
/// self explanatory
#[derive(Debug)]
pub struct Function {
    
}

// Lua primitives, String is defined in mlua
pub type SizeT = usize;
pub type Boolean = bool;
pub type Integer = i32;
pub type Number = c_double;

/// The is_vararg flag
#[derive(Debug)]
pub enum Vararg {
    HasArg,
    IsVararg,
    NeedsVararg
}

/// A lua constant
#[derive(Debug)]
pub enum Constant<'a> {
    Nil,
    Boolean(Boolean),
    Number(Number),
    String(mlua::String<'a>)
}