use super::{RispErr, RispExpr};

pub fn parse(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| RispErr::Reason("Could not get token".into()))?;

    match token.as_str() {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("Unexpected ')'".into())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn parse_all(tokens: &[String]) -> Result<Vec<RispExpr>, RispErr> {
    let mut exprs = vec![];
    let mut input = tokens;

    while !input.is_empty() {
        let (expr, rest) = parse(input)?;

        exprs.push(expr);

        input = rest;
    }

    Ok(exprs)
}

fn read_seq(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
    let mut res: Vec<RispExpr> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| RispErr::Reason("could not find closing ')'".into()))?;

        if next_token == ")" {
            return Ok((RispExpr::List(res), rest));
        }

        let (expr, new_xs) = parse(&xs)?;

        res.push(expr);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> RispExpr {
    match token {
        "true" => RispExpr::Bool(true),
        "false" => RispExpr::Bool(false),
        _ => {
            let potential_number = token.parse::<f64>();
            match potential_number {
                Ok(value) => RispExpr::Number(value),
                Err(_) => {
                    if token.starts_with(':') {
                        RispExpr::Keyword(token.to_string())
                    } else {
                        RispExpr::Symbol(token.to_string())
                    }
                }
            }
        }
    }
}
