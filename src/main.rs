mod instr;
mod parse;
mod vmstr;

use mlua::Lua;

use instr::Constant;
use instr::Function;

use std::fs::write;

fn debug_func(func: &Function, level: u32) {
    let indent = " ".repeat(level as usize);
    print!("{}constants: ", indent);
    for c in &func.constants {
        match c {
            Constant::String(s) => {
                print!("{}{}, ", indent, s.to_string_lossy())
            }
            _ => {
                print!("{}{:?}, ", indent, c);
            }
        }
    }
    println!();
    for i in &func.instructions {
        match i.instruction_kind {
            instr::InstructionKind::ABC => {
                println!("{}{:>10?}{:>4}{:>4}{:>4}", indent, i.op_code, i.a, i.b.unwrap(), i.c.unwrap());
            },
            instr::InstructionKind::ABx => {
                println!("{}{:10?}{:4}{:4}", indent, i.op_code, i.a, i.bx.unwrap());
            },
            instr::InstructionKind::AsBx => {
                println!("{}{:10?}{:4}{:4}", indent, i.op_code, i.a, i.sbx.unwrap());
            },
        }
    }
    for f in &func.function_protos {
        debug_func(f, level + 1);
    }
}

fn main() {
    let state = Lua::new();
    let src = String::from(r#"
        print(15, 92.2441, true)
	"#);
    let func = parse::deserialize(&src, &state).unwrap();
    debug_func(&func, 0);

}
