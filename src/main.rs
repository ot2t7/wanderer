mod vm_constants;

use vm_constants::{VM, BASE64_DECODE, VM_CALL};
use mlua::Lua;
use std::fs;
use base64::encode;

const IN: &str = "in.lua";
const OUT: &str = "out.lua";

fn main() {
    let state = Lua::new();
    let prerequisite = format!("{}{}", VM, BASE64_DECODE);

    // Compile inputted lua code
    let src = fs::read_to_string(IN)
        .expect("Something went wrong reading the file");
    let bc = state.load(&src).into_function()
        .expect("Failed compiling the source code").dump(true);

    // Construct a call to the vm
    let call = VM_CALL.replace("$BYTECODE", encode(bc).as_str());
    
    // Write the final result
    fs::write(OUT, format!("{}{}", prerequisite, call))
        .expect("Couldn't write to output file");
}
