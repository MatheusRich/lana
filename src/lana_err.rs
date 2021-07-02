use super::Token;

#[derive(Debug, PartialEq)]
pub enum LanaErr {
    Reason(String),
    UnknownToken(Token),
}

impl std::fmt::Display for LanaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            LanaErr::Reason(msg) => msg.to_string(),
            LanaErr::UnknownToken(token) => format!("unknown token {:?}", token),
        };

        write!(f, "{}", string)
    }
}