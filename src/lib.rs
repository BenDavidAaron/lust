use std::collections::HashMap;

#[derive(Debug)]
enum LustExpression {
    Symbol(String),
    Number(f64),
    List(Vec<LustExpression>),
}

#[derive(Debug)]
enum LustException {
    Reason(String),
}

struct LustEnv {
    data: HashMap<String, LustExpression>,
}

pub fn tokenize(expression: String) -> Vec<String> {
    expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const SIMPLE_PROGRAM: &str = "(+ 2 2)";
    #[test]
    fn tokenize_simple_program() {
        assert_eq!(tokenize(SIMPLE_PROGRAM.to_string()), vec!["(", "+", "2", "2", ")"])
    }
}
