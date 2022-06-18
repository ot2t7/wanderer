mod instr;
mod parse;

fn main() {
    let src = String::from(r#"
		local a = 5;
		local function haha()
			print(a);
		end
	"#);
    parse::deserialize(&src).unwrap();
}
