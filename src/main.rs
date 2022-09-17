use lust::*;

fn main() {
    let program = "(* 3 (+ 2 2))";
    println!("{}", program);
    let tokens = tokenize(program.to_string());
    println!("{:?}", tokens);
}
