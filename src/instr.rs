use std::os::raw::c_double;

/// Every single Lua 5.1 OpCode
#[derive(Debug)]
pub enum Instruction {
    Move(iABC),
    Loadk(iABx), 
    LoadBool(iABC), 
    LoadNil(iABC), 
    GetUpval(iABC), 
    GetGlobal(iABx), 
    GetTable(iABC), 
    SetGlobal(iABx), 
    SetUpval(iABC), 
    SetTable(iABC), 
    NewTable(iABC), 
    _Self(iABC), // Self is already reserved by rust
    Add(iABC), 
    Sub(iABC), 
    Mul(iABC), 
    Div(iABC), 
    Mod(iABC), 
    Pow(iABC), 
    Unm(iABC), 
    Not(iABC), 
    Len(iABC), 
    Concat(iABC), 
    Jmp(iAsBx), 
    Eq(iABC), 
    Lt(iABC), 
    Le(iABC), 
    Test(iABC), 
    TestSet(iABC), 
    Call(iABC), 
    TailCall(iABC), 
    Return(iABC), 
    ForLoop(iAsBx), 
    ForPrep(iAsBx), 
    TForLoop(iABC), 
    SetList(iABC), 
    Close(iABC), 
    Closure(iABx), 
    Vararg(iABC),
    Unknown
}

// Instruction formats, note that the sizes of the properties do not
// reflect their actual size in the bytecode.

#[derive(Debug, Default)]
pub struct iABC {
    A: u8,
    C: u16,
    B: u16
}

#[derive(Debug, Default)]
pub struct iABx {
    A: u8,
    Bx: u32,
}

#[derive(Debug, Default)]
pub struct iAsBx {
    A: u8,
    sBx: i32
}

/// An instance of this struct represents a deserialized function block
/// that has been extracted from the inputted bytecode. Most fields are
/// self explanatory.
#[derive(Debug)]
pub struct Function {
    
}

// Lua primitives, String is defined in mlua
pub type size_t = usize;
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