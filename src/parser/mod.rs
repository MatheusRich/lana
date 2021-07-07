mod lana_expr;

use super::{LanaErr, Token, TokenKind};
pub use lana_expr::{LanaExpr, LanaLambda};

pub fn parse(tokens: &[Token]) -> Result<(LanaExpr, &[Token]), LanaErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| LanaErr::Reason("Could not get token".into()))?;

    match token.kind {
        TokenKind::LParen => read_seq(rest, token.clone()),
        TokenKind::RParen => Err(LanaErr::UnexpectedToken(token.clone())),
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

fn read_seq(tokens: &[Token], opening_token: Token) -> Result<(LanaExpr, &[Token]), LanaErr> {
    let mut res: Vec<LanaExpr> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| LanaErr::UnterminatedExpr((')', opening_token.clone())))?;

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
        TokenKind::UnterminatedString(_) => Err(LanaErr::UnterminatedExpr(('"', token.clone()))),
        _ => panic!("Cannot parse atom from token {:?}", token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::SrcLocation;

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
    fn it_parses_false() {
        let input = vec![Token::new(
            TokenKind::Id("false".into()),
            SrcLocation::new(1, 3),
        )];

        let (result, _) = parse(&input).expect("Could not parse false");

        assert_eq!(LanaExpr::Bool(false), result);
    }

    #[test]
    fn it_parses_true() {
        let input = vec![Token::new(
            TokenKind::Id("true".into()),
            SrcLocation::new(1, 3),
        )];

        let (result, _) = parse(&input).expect("Could not parse true");

        assert_eq!(LanaExpr::Bool(true), result);
    }

    #[test]
    fn it_parses_ids() {
        let input = vec![
            Token::new(TokenKind::Id("my-var".into()), SrcLocation::new(1, 5)),
            Token::new(TokenKind::Id("my_var".into()), SrcLocation::new(2, 5)),
        ];

        let result = parse_all(&input).expect("Could not parse id");

        assert_eq!(
            vec![
                LanaExpr::Symbol("my-var".to_string()),
                LanaExpr::Symbol("my_var".to_string())
            ],
            result
        );
    }

    #[test]
    fn it_parses_keywords() {
        let input = vec![
            Token::new(TokenKind::Id(":my-var".into()), SrcLocation::new(1, 6)),
            Token::new(TokenKind::Id(":1".into()), SrcLocation::new(2, 2)),
        ];

        let result = parse_all(&input).expect("Could not parse id");

        assert_eq!(
            vec![
                LanaExpr::Keyword(":my-var".to_string()),
                LanaExpr::Keyword(":1".to_string())
            ],
            result
        );
    }

    #[test]
    fn it_parses_numbers() {
        let input = vec![
            Token::new(TokenKind::Number(1.0), SrcLocation::new(1, 3)),
            Token::new(TokenKind::Number(-1.0), SrcLocation::new(2, 3)),
        ];

        let tokens = parse_all(&input).expect("Could not parse number");

        assert_eq!(vec![LanaExpr::Number(1.0), LanaExpr::Number(-1.0)], tokens);
    }

    #[test]
    fn it_parses_strings() {
        let input = vec![Token::new(
            TokenKind::String("hello world".into()),
            SrcLocation::new(1, 11),
        )];

        let (result, _) = parse(&input).expect("Could not parse string");

        assert_eq!(LanaExpr::String("hello world".into()), result);
    }

    #[test]
    fn it_parses_an_empty_list() {
        let input = vec![
            Token::new(TokenKind::LParen, SrcLocation::new(1, 1)),
            Token::new(TokenKind::RParen, SrcLocation::new(1, 2)),
        ];

        let result = parse_all(&input).expect("Could not parse empty list");

        assert_eq!(vec![LanaExpr::List(vec![])], result);
    }

    #[test]
    fn it_parses_a_list() {
        let input = vec![
            Token::new(TokenKind::LParen, SrcLocation::new(1, 1)),
            Token::new(TokenKind::Number(1.0), SrcLocation::new(1, 2)),
            Token::new(TokenKind::RParen, SrcLocation::new(1, 3)),
        ];

        let result = parse_all(&input).expect("Could not parse a list");

        assert_eq!(vec![LanaExpr::List(vec![LanaExpr::Number(1.0)])], result);
    }

    #[test]
    fn it_errors_on_unterminated_lists() {
        let opening_paren = Token::new(TokenKind::LParen, SrcLocation::new(1, 1));
        let input = vec![
            opening_paren.clone(),
            Token::new(TokenKind::Number(1.0), SrcLocation::new(1, 2)),
        ];

        let result = parse_all(&input).expect_err("Didn't fail on unterminated list");

        assert_eq!(LanaErr::UnterminatedExpr((')', opening_paren)), result);
    }

    #[test]
    fn it_errors_on_unterminated_strings() {
        let unterminated_string = Token::new(
            TokenKind::UnterminatedString("\"unterminated".to_string()),
            SrcLocation::new(1, 1),
        );
        let input = vec![unterminated_string.clone()];

        let result = parse_all(&input).expect_err("Didn't fail on unterminated string");

        assert_eq!(
            LanaErr::UnterminatedExpr(('"', unterminated_string)),
            result
        );
    }

    #[test]
    fn it_errors_on_unexpected_parethesis() {
        let input = vec![Token::new(TokenKind::RParen, SrcLocation::new(1, 1))];

        let result = parse_all(&input).expect_err("Didn't failed on unexpected token");

        assert_eq!(
            LanaErr::UnexpectedToken(Token::new(TokenKind::RParen, SrcLocation::new(1, 1))),
            result
        );
    }

    #[test]
    #[should_panic(expected = "Cannot parse atom from token ')' at line 1, column 1")]
    fn it_panics_on_invalid_atoms() {
        let input = Token::new(TokenKind::RParen, SrcLocation::new(1, 1));

        parse_atom(&input).expect_err("Didn't panic on unexpected token");
    }
}
