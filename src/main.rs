mod instr;
mod parse;

use mlua::Lua;

use instr::Constant;
use instr::Function;

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
    for f in &func.function_protos {
        debug_func(f, level + 1);
    }
}

fn main() {
    let state = Lua::new();
    let src = String::from(r#"

	"#);
    let func = parse::deserialize(&src, &state).unwrap();
    debug_func(&func, 0);
}
