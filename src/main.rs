use lust::{tokenize, parse};

fn main() {
    const PROGRAM: &str = "( + 7 (+ 69 420))";
    println!("program: {}", PROGRAM.to_string());
    let tokens = tokenize(PROGRAM.to_string()); //  returns a list of tokens
    println!("tokens: {:?}", tokens);
    let expression = parse(&tokens);    //  converts tokens to exprs of atoms/exprs
    println!("expression: {:?}", expression)
}
