fn tokenize(chars: &str) -> Vec<str>{
    let left_padded = str::replace(chars, "(", " ( ");
    let right_padded = str::replace(&left_padded, ")", " ) ");
    let tokens: Vec<&str> = right_padded.split_whitespace().collect();
    return tokens
}

fn main() {
    let program = "(* 3 (+ 2 2))";
    println!("{}", program);
    let tokens = tokenize(program);
    println!("{}", tokens);
}

