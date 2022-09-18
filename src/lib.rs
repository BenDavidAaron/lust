use std::{collections::HashMap, num::ParseFloatError};

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

fn tokenize(expression: String) -> Vec<String> {
    expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(LustExpression, &'a [String]), LustException> {
    let (token, rest) = tokens.split_first().ok_or(LustException::Reason(
        "Bruh, I can't get the next Token".to_string(),
    ))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(LustException::Reason(
            "Bruh you put `)` in the wrong place!".to_string(),
        )),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(LustExpression, &'a [String]), LustException> {
    let mut res: Vec<LustExpression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(LustException::Reason(
            "Bruh you forgot a `)` somewhere!".to_string(),
        ))?;
        if next_token == ")" {
            return Ok((LustExpression::List(res), rest)); // skip closing parens, go to next
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> LustExpression {
    let potential_float: Result<f64, ParseFloatError>= token.parse();
    match potential_float {
        Ok(v) => LustExpression::Number(v),
        Err(_) => LustExpression::Symbol(token.to_string().clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SIMPLE_PROGRAM: &str = "(+ 2 2)";
    #[test]
    fn tokenize_simple_program() {
        assert_eq!(
            tokenize(SIMPLE_PROGRAM.to_string()),
            vec!["(", "+", "2", "2", ")"]
        )
    }
}
