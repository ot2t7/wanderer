mod instr;
mod parse;

fn main() {
    let src = String::from("print('hi')");
    parse::deserialize(&src).unwrap();
}
