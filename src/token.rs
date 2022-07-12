use std::collections::HashMap;
use std::ffi::OsString;

use departure::{Constant, Function, Instruction, Integer, Local, OpCode, Vararg};
use rand::random;
use strum::IntoEnumIterator;

type Token = i32; // Keep in mind: the usual size of a lua number is 8 bytes, or 64 bits.

pub struct TokenizedInstruction {
    instruction: Instruction,
    current_token: Token,
    token_offsets: Vec<Token>,
}

pub struct TokenizedFunction {
    pub source_name: OsString,
    pub line_defined: Integer,
    pub last_line_defined: Integer,
    pub num_upvalues: u8,
    pub num_parameters: u8,
    pub is_vararg: Vararg,
    pub stack_size: u8,
    pub instructions: Vec<TokenizedInstruction>,
    pub constants: Vec<Constant>,
    pub function_protos: Vec<TokenizedFunction>,
    // Debug data
    pub instruction_positions: Vec<Integer>,
    pub name_locals: Vec<Local>,
    pub name_upvalues: Vec<OsString>,
}

/// Generate a unique token for every single possible opcode
pub fn tokenize() -> HashMap<OpCode, Token> {
    let mut to_return = HashMap::new();
    for op in OpCode::iter() {
        to_return.insert(op, random::<Token>());
    }
    return to_return;
}

pub fn register_tokens(func: Function, token_defs: HashMap<OpCode, Token>) -> TokenizedFunction {
    let tokenized: Vec<TokenizedInstruction> = vec![];
    let tokenized_protos: Vec<TokenizedFunction> = vec![];
    let current_token: Token = 0;

    for (i, instr) in func.instructions.iter().enumerate() {
        
    }

    return TokenizedFunction {
        source_name: func.source_name,
        line_defined: func.line_defined,
        last_line_defined: func.last_line_defined,
        num_upvalues: func.num_upvalues,
        num_parameters: func.num_parameters,
        is_vararg: func.is_vararg,
        stack_size: func.stack_size,
        instructions: tokenized,
        constants: func.constants,
        function_protos: tokenized_protos,
        instruction_positions: func.instruction_positions,
        name_locals: func.name_locals,
        name_upvalues: func.name_upvalues,
    };
}
