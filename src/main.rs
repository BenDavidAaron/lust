use lust::{tokenize, parse};

fn main() {
    const PROGRAM: &str = "( + 7 (+ 69 420))";
    println!("program: {}", PROGRAM.to_string());
    let tokens = tokenize(PROGRAM.to_string());
    println!("tokens: {:?}", tokens);
    let expression = parse(&tokens);
    println!("expression: {:?}", expression)
}
