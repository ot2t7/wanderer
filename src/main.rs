mod instr;
mod parse;

use std::io::{Cursor, Read};

fn main() {
    let src = String::from("print('hi')");
    parse::deserialize(&src).unwrap();
}
