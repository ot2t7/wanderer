mod instr;
mod parse;

fn main() {
    let src = String::from(r#"

	"#);
    parse::deserialize(&src).unwrap();
}
