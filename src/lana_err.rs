use super::Token;

#[derive(Debug, PartialEq)]
pub enum LanaErr {
    Reason(String),
    UnexpectedToken(Token),
    UnterminatedExpr((char, Token)),
}

impl std::fmt::Display for LanaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            LanaErr::Reason(msg) => msg.to_string(),
            LanaErr::UnexpectedToken(token) => format!("unexpected {:?}", token),
            LanaErr::UnterminatedExpr((expected, opening_token)) => {
                format!(
                    "could not find closing '{}' for {:?}",
                    expected, opening_token
                )
            }
        };

        write!(f, "{}", string)
    }
}
