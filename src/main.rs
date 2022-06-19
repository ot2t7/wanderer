mod instr;
mod parse;

use mlua::Lua;

fn main() {
    let state = Lua::new();
    let src = String::from(r#"
        local lmao = "aaaa"
        print(lmao)
        local function dog()
            print("Thats so dog dude" .. lmao)
        end
	"#);
    parse::deserialize(&src, &state).unwrap();
}
