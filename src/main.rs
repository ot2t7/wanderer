mod token;
mod vmstr;

use token::tokenize;

fn main() {
    let codes = tokenize();
    println!("{:?}", codes);
}