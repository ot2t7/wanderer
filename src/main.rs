mod token;
mod vmstr;
mod debug;

use token::{tokenize, register_tokens};
use debug::debug_tokens;

use departure::deserialize;

fn main() {
    //println!("{:<12}{:<6}{:<6}{:<6}", "GetGlobal", 1, 0, 1);
    //println!("{:<12}{:<6}{:<6}", "Return", 0, 0);

    /* 
    let source = String::from(r#"
        if 5 > 10 then
            print("wtf")
        end
    "#);
    let compiled = deserialize(&source).unwrap();

    let codes= tokenize();
    let tokenized = register_tokens(&compiled, &codes);
    debug_tokens(&tokenized);
    */

    println!("{:?}", vmstr::load_vm_strings());
}