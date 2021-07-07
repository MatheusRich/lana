use super::SrcLocation;

#[derive(PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: SrcLocation,
}

impl Token {
    pub fn new(kind: TokenKind, loc: SrcLocation) -> Self {
        Self { kind, loc }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}", self.kind, self.loc)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    String(String),
    Number(f64),
    LParen,
    RParen,
    Id(String),
    UnterminatedString(String),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            TokenKind::String(s) => format!("'{}'", s),
            TokenKind::Number(n) => format!("'{}'", n),
            TokenKind::Id(k) => format!("'{}'", k),
            TokenKind::UnterminatedString(token) => format!("'{}'", token),
            TokenKind::LParen => "'('".to_string(),
            TokenKind::RParen => "')'".to_string(),
        };

        write!(f, "{}", string)
    }
}
