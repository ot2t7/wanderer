use crate::instr::Function;

use rand::random;

/// Take in a vector of instructions in a func, and for every single
/// instruction generate an instruction token
pub fn tokenize(func: &mut Function) {
    for i in func.instructions.iter_mut() {
        i.token = Some(random::<i64>());
    }
    for f in func.function_protos.iter_mut() {
        tokenize(f);
    }
}