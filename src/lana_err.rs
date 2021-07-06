use super::Token;

#[derive(Debug, PartialEq)]
pub enum LanaErr {
    Reason(String),
    UnterminatedString(Token),
    UnexpectedToken(Token),
}

impl std::fmt::Display for LanaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            LanaErr::Reason(msg) => msg.to_string(),
            LanaErr::UnterminatedString(token) => format!("unterminated string {:?}", token),
            LanaErr::UnexpectedToken(token) => format!("unexpected {:?}", token),
        };

        write!(f, "{}", string)
    }
}
