use std::{os::raw::c_double, fmt};

/// Every single Lua 5.1 OpCode
pub enum Instruction {
    
}

// Instruction formats, note that the sizes of the properties do not
// reflect their actual size in the bytecode.

pub struct iABC {
    A: u8,
    C: u16,
    B: u16
}

pub struct iABx {
    A: u8,
    Bx: u32,
}

pub struct iAsBx {
    A: u8,
    Bx: i32
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