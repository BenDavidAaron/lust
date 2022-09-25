use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug)]
pub enum LustExpression {
    Symbol(String),
    Number(f64),
    List(Vec<LustExpression>),
}

#[derive(Debug)]
pub enum LustException {
    Reason(String),
}

pub struct LustEnv {
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

/*
def read(ts):
    if len(ts) == 0:
        raise SyntaxError('unexpected EOF')
    t = ts.pop(0)
    if t == "(":
        l = []
        while ts[0] != ")":
            l.append(read(ts))
        ts.pop(0)
        return l
    elif t == ")":
        raise SyntaxError("unexpected )")
    else:
        return atom(t)
*/

pub fn parse<'a>(tokens: &'a [String]) -> Result<(LustExpression, &'a [String]), LustException> {
    for token in tokens.iter(){
        match token {
            "(" => { // push new list to stack, inc depth
                let mut res: Vec<LustExpression> = vec![];
                // loop through tokens until ), then break and read the remaining tokens (if there's any)
                res.push(parse( /*parse remaining tokens*/ ))
                
            }
            ")" => {
                res.pop();
                depth -= 1;
            }
            // ")"  // pop the list from stack and push into main list at depth - 1
            //      if depth is 0, error
            //  other token, push onto current expr

    }
}

(+ (+ 1 2) (+ 3 4))
[+ 
    [+ 1 2]

]

pub fn read_seq<'a>(tokens: &'a [String]) -> Result<(LustExpression, &'a [String]), LustException> {
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

pub fn parse_atom(token: &str) -> LustExpression {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => LustExpression::Number(v),
        Err(_) => LustExpression::Symbol(token.to_string().clone()),
    }
}
