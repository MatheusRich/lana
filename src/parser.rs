use super::{LanaErr, LanaExpr, Token, TokenKind};

pub fn parse(tokens: &[Token]) -> Result<(LanaExpr, &[Token]), LanaErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| LanaErr::Reason("Could not get token".into()))?;

    match token.kind {
        TokenKind::LParen => read_seq(rest),
        TokenKind::RParen => Err(LanaErr::Reason("Unexpected ')'".into())),
        _ => Ok((parse_atom(token)?, rest)),
    }
}

pub fn parse_all(tokens: &[Token]) -> Result<Vec<LanaExpr>, LanaErr> {
    let mut exprs = vec![];
    let mut input = tokens;

    while !input.is_empty() {
        let (expr, rest) = parse(input)?;

        exprs.push(expr);

        input = rest;
    }

    Ok(exprs)
}

fn read_seq(tokens: &[Token]) -> Result<(LanaExpr, &[Token]), LanaErr> {
    let mut res: Vec<LanaExpr> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| LanaErr::Reason("could not find closing ')'".into()))?;

        if next_token.kind == TokenKind::RParen {
            return Ok((LanaExpr::List(res), rest));
        }

        let (expr, new_xs) = parse(&xs)?;

        res.push(expr);
        xs = new_xs;
    }
}

fn parse_atom(token: &Token) -> Result<LanaExpr, LanaErr> {
    match &token.kind {
        TokenKind::Number(n) => Ok(LanaExpr::Number(*n)),
        TokenKind::String(s) => Ok(LanaExpr::String(s.clone())),
        TokenKind::Id(value) if value == "true" => Ok(LanaExpr::Bool(true)),
        TokenKind::Id(value) if value == "false" => Ok(LanaExpr::Bool(false)),
        TokenKind::Id(value) if value == "nil" => Ok(LanaExpr::Nil),
        TokenKind::Id(value) if value.starts_with(':') => Ok(LanaExpr::Keyword(value.clone())),
        TokenKind::Id(value) => Ok(LanaExpr::Symbol(value.clone())),
        TokenKind::Unknown(_) => Err(LanaErr::UnknownToken(token.clone())),
        _ => panic!("Cannot parse atom from token {:?}", token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_nil() {
        let input = vec![Token::new(
            TokenKind::Id("nil".into()),
            SrcLocation::new(1, 3),
        )];

        let (result, _) = parse(&input).expect("Could not parse nil");

        assert_eq!(LanaExpr::Nil, result);
    }

    #[test]
    fn it_parses_a_number() {
        let input = vec![Token::new(
            TokenKind::Number(1.0),
            SrcLocation::new(1, 3),
        )];

        let (result, _) = parse(&input).expect("Could not parse number");

        assert_eq!(LanaExpr::Number(1.0), result);
    }
}
