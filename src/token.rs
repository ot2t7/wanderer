use std::collections::HashMap;

use departure::OpCode;
use rand::random;
use strum::IntoEnumIterator;

type Token = i32; // Keep in mind: the usual size of a lua number is 8 bytes, or 64 bits.

/// Generate a unique token for every single possible opcode
pub fn tokenize() -> HashMap<OpCode, Token> {
    let mut to_return = HashMap::new();
    for op in OpCode::iter() {
        to_return.insert(op, random::<Token>());
    }
    return to_return;
}