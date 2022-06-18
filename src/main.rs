mod instr;
mod parse;

fn main() {
    let src = String::from(r#"
		local a = 5;
		print(a);
	"#);
    parse::deserialize(&src).unwrap();
}
