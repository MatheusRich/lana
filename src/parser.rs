use super::{LanaErr, LanaExpr};

pub fn parse(tokens: &[String]) -> Result<(LanaExpr, &[String]), LanaErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| LanaErr::Reason("Could not get token".into()))?;

    match token.as_str() {
        "(" => read_seq(rest),
        ")" => Err(LanaErr::Reason("Unexpected ')'".into())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn parse_all(tokens: &[String]) -> Result<Vec<LanaExpr>, LanaErr> {
    let mut exprs = vec![];
    let mut input = tokens;

    while !input.is_empty() {
        let (expr, rest) = parse(input)?;

        exprs.push(expr);

        input = rest;
    }

    Ok(exprs)
}

fn read_seq(tokens: &[String]) -> Result<(LanaExpr, &[String]), LanaErr> {
    let mut res: Vec<LanaExpr> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| LanaErr::Reason("could not find closing ')'".into()))?;

        if next_token == ")" {
            return Ok((LanaExpr::List(res), rest));
        }

        let (expr, new_xs) = parse(&xs)?;

        res.push(expr);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> LanaExpr {
    match token {
        "true" => LanaExpr::Bool(true),
        "false" => LanaExpr::Bool(false),
        "nil" => LanaExpr::Nil,
        _ => {
            let potential_number = token.parse::<f64>();
            match potential_number {
                Ok(value) => LanaExpr::Number(value),
                Err(_) => {
                    if token.starts_with(':') {
                        LanaExpr::Keyword(token.to_string())
                    } else {
                        LanaExpr::Symbol(token.to_string())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_nil() {
        let input = vec![String::from("nil")];

        let (result, _) = parse(&input).expect("Could not parse nil");

        assert_eq!(LanaExpr::Nil, result);
    }
}
