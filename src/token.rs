use std::collections::HashMap;
use std::ffi::OsString;

use departure::{Constant, Function, Instruction, Integer, Local, OpCode, Vararg};
use rand::random;
use strum::IntoEnumIterator;

pub type Token = i32; // Keep in mind: the usual size of a lua number is 8 bytes, or 64 bits.
pub type RToken = i64; // A bigger token type to represent multiple tokens added.

#[derive(PartialEq, Clone)]
pub struct TokenizedInstruction {
    pub instruction: Instruction,
    pub current_token: RToken,
    pub token_offsets: Vec<RToken>,
}

#[derive(Clone)]
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

pub fn register_tokens(func: &Function, token_defs: &HashMap<OpCode, Token>) -> TokenizedFunction {
    let mut tokenized: Vec<TokenizedInstruction> = vec![];
    let mut tokenized_protos: Vec<TokenizedFunction> = vec![];
    let mut current_token: RToken = 0;

    for (i, instr) in func.instructions.iter().enumerate() {
        // All instructions which jump:
        // LoadBool, Jmp, Eq, Lt, Le, Test, Testset
        match instr.op_code { // Doing instructions[i + 1] is safe because last instruction
                              // is always going to be Return
            OpCode::LoadBool | OpCode::Eq | OpCode::Lt | OpCode::Le | OpCode::Test | OpCode::TestSet | OpCode::TForLoop => {
                let mut pc_inc = token_defs[&func.instructions[i + 1].op_code] as RToken;
                let offset = token_defs[&instr.op_code] as RToken;
                pc_inc += offset;
                let orig_instr = (*instr).clone();
                tokenized.push(TokenizedInstruction { 
                    instruction: orig_instr, 
                    current_token: current_token, 
                    token_offsets: vec![offset, pc_inc]
                });
                current_token += offset;
            }
            OpCode::ForPrep | OpCode::ForLoop | OpCode::Jmp => {
                let offset = token_defs[&instr.op_code] as RToken;
                let to_jump = instr.sbx.unwrap() as RToken;
                let mut skipped_offsets: RToken = 0;
                // Rust can't forloop with a negative increment, so we're gonna have to do
                // some hacks
                if to_jump < 0 {
                    for s in (i as RToken + to_jump)..(i as RToken) {
                        skipped_offsets -= token_defs[&func.instructions[s as usize].op_code] as RToken;
                    }
                } else {
                    skipped_offsets += offset;
                    for s in (i as RToken + 1)..(i as RToken + to_jump + 1) {
                        skipped_offsets += token_defs[&func.instructions[s as usize].op_code] as RToken;
                    }
                }
                let orig_instr = (*instr).clone();
                tokenized.push(TokenizedInstruction { 
                    instruction: orig_instr, 
                    current_token: current_token, 
                    token_offsets: if instr.op_code != OpCode::Jmp { vec![offset, skipped_offsets] } else { vec![skipped_offsets] }
                });
                current_token += offset;
            }
            _ => { // Non jumping instruction
                let offset = token_defs[&instr.op_code] as RToken;
                let orig_instr = (*instr).clone();
                tokenized.push(TokenizedInstruction { 
                    instruction: orig_instr, 
                    current_token: current_token,
                    token_offsets: vec![offset]
                });
                current_token += offset;
            }
        }
    }

    for f in &func.function_protos {
        tokenized_protos.push(register_tokens(f, token_defs));
    }

    return TokenizedFunction {
        source_name: func.source_name.clone(),
        line_defined: func.line_defined,
        last_line_defined: func.last_line_defined,
        num_upvalues: func.num_upvalues,
        num_parameters: func.num_parameters,
        is_vararg: func.is_vararg,
        stack_size: func.stack_size,
        instructions: tokenized,
        constants: func.constants.clone(),
        function_protos: tokenized_protos,
        instruction_positions: func.instruction_positions.clone(),
        name_locals: func.name_locals.clone(),
        name_upvalues: func.name_upvalues.clone(),
    };
}
