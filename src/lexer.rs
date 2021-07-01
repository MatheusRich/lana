#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: SrcLocation,
}

impl Token {
    fn new(kind: TokenKind, loc: SrcLocation) -> Self {
        Self { kind, loc }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SrcLocation {
    pub line: i32,
    pub col: i32,
}

impl SrcLocation {
    fn default() -> Self {
        SrcLocation { line: 1, col: 0 }
    }

    fn new(line: i32, col: i32) -> Self {
        SrcLocation { line, col }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    String(String),
    Number(f64),
    LParen,
    RParen,
    Id(String),
    Unknown(String),
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    src: std::iter::Peekable<std::str::Chars<'a>>,
    loc: SrcLocation,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            loc: SrcLocation::default(),
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        self.next_char().and_then(|c| match c {
            '(' => Some(Token::new(TokenKind::LParen, self.loc())),
            ')' => Some(Token::new(TokenKind::RParen, self.loc())),
            ';' => {
                self.skip_line();
                self.next_token()
            }
            '"' => Some(self.read_string()),
            c => Some(self.read_id_or_number(c)),
        })
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() || *c == ',' {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Token {
        let mut string: String = String::new();
        let token: Token;

        loop {
            match self.next_char() {
                Some('"') => {
                    token = Token::new(TokenKind::String(string), self.loc());
                    break;
                }
                Some(c) => match c {
                    '\\' => match self.next_char() {
                        Some('n') => string.push('\n'),
                        Some('t') => string.push('\t'),
                        Some('"') => string.push('"'),
                        Some(c) => string.push(c),
                        None => continue,
                    },
                    _ => string.push(c),
                },
                None => {
                    token = Token::new(TokenKind::Unknown(string), self.loc());
                    break;
                }
            };
        }

        token
    }

    fn read_id_or_number(&mut self, begin: char) -> Token {
        let mut token: String = String::from(begin);

        loop {
            let is_separator = self.src.peek().map_or(true, |c| Self::is_separator(*c));

            if is_separator {
                break;
            } else {
                let c = self.next_char().expect("Should have a separator character");
                token.push(c);
            }
        }

        match token.parse::<f64>() {
            Ok(n) => Token::new(TokenKind::Number(n), self.loc()),
            Err(_) => Token::new(TokenKind::Id(token), self.loc()),
        }
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.next_char() {
            if c == '\n' {
                break;
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let chr = self.src.next();

        if let Some(c) = chr {
            self.update_loc(c);
        }

        chr
    }

    fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn update_loc(&mut self, c: char) {
        match c {
            '\n' => {
                self.loc.line += 1;
                self.loc.col = 0;
            }
            _ => self.loc.col += 1,
        }
    }

    fn loc(&self) -> SrcLocation {
        self.loc.clone()
    }

    fn is_separator(c: char) -> bool {
        match c {
            '(' | ')' | ';' => true,
            _ => c.is_whitespace(),
        }
    }
}

//////////////////////////////// OLD CODE ////////////////////////////////

pub fn tokenize(code: String) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .replace(",", " ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec_eq<T>(va: &[T], vb: &[T])
    where
        T: std::fmt::Debug + std::cmp::PartialEq,
    {
        assert_eq!(va.len(), vb.len(), "Vectors have different lengths");

        for (a, b) in va.iter().zip(vb) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn it_parses_an_empty_input() {
        let input = "".to_string();
        let mut lexer = Tokenizer::new(&input);

        let tokens = lexer.tokens();

        assert!(tokens.is_empty())
    }

    #[test]
    fn it_parses_multiple_tokens() {
        let input = "(+ 1 2 ; yay \n )".to_string();
        let mut lexer = Tokenizer::new(&input);

        let tokens = lexer.tokens();

        assert_vec_eq(
            &[
                Token::new(TokenKind::LParen, SrcLocation::new(1, 1)),
                Token::new(TokenKind::Id("+".to_string()), SrcLocation::new(1, 2)),
                Token::new(TokenKind::Number(1.0), SrcLocation::new(1, 4)),
                Token::new(TokenKind::Number(2.0), SrcLocation::new(1, 6)),
                Token::new(TokenKind::RParen, SrcLocation::new(2, 2)),
            ],
            &tokens,
        );
    }

    #[test]
    fn it_ignores_whitespaces() {
        let input = " \ta".to_string();
        let mut lexer = Tokenizer::new(&input);

        lexer.next_token();

        assert_eq!(1, lexer.loc.line);
        assert_eq!(3, lexer.loc.col);
    }

    #[test]
    fn it_ignores_commas() {
        let input = ",,,".to_string();
        let mut lexer = Tokenizer::new(&input);

        lexer.next_token();

        assert_eq!(1, lexer.loc.line);
        assert_eq!(3, lexer.loc.col);
    }

    #[test]
    fn it_skips_comments() {
        let input = "; some comment here\n)".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to comments");

        assert_eq!(
            Token {
                kind: TokenKind::RParen,
                loc: SrcLocation { line: 2, col: 1 }
            },
            token
        );
    }

    #[test]
    fn it_update_location_line_on_new_line_chars() {
        let input = "\n  \n   ".to_string();
        let mut lexer = Tokenizer::new(&input);

        lexer.next_token();

        assert_eq!(3, lexer.loc.line);
        assert_eq!(3, lexer.loc.col);
    }

    #[test]
    fn it_lexes_a_left_paren() {
        let input = "(".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex a left paren");

        assert_eq!(
            Token {
                kind: TokenKind::LParen,
                loc: SrcLocation { line: 1, col: 1 }
            },
            token
        );
    }

    #[test]
    fn it_lexes_a_right_paren() {
        let input = ")".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex a right paren");

        assert_eq!(
            Token {
                kind: TokenKind::RParen,
                loc: SrcLocation { line: 1, col: 1 }
            },
            token
        );
    }

    #[test]
    fn it_lexes_strings() {
        let input = "\"hello world!\"".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex strings");

        assert_eq!(
            Token {
                kind: TokenKind::String("hello world!".to_string()),
                loc: SrcLocation { line: 1, col: 14 }
            },
            token
        );
    }

    #[test]
    fn it_lexes_strings_with_escapes() {
        let input = r#" "hello\nworld\t  \"!" "#.to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex strings");

        assert_eq!(
            Token {
                kind: TokenKind::String("hello\nworld\t  \"!".to_string()),
                loc: SrcLocation { line: 1, col: 22 }
            },
            token
        );
    }

    #[test]
    fn it_ignores_unknown_escapes() {
        let input = "\"hello\\çworld!\"".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex strings");

        assert_eq!(
            Token {
                kind: TokenKind::String("helloçworld!".to_string()),
                loc: SrcLocation { line: 1, col: 15 }
            },
            token
        );
    }

    #[test]
    fn it_parses_unterminated_string() {
        let input = "\"hello\\".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex unterminated string");

        assert_eq!(
            Token {
                kind: TokenKind::Unknown("hello".to_string()),
                loc: SrcLocation { line: 1, col: 7 }
            },
            token
        );
    }

    #[test]
    fn it_parses_simple_numbers() {
        let input = "123".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Number(123.0),
                loc: SrcLocation { line: 1, col: 3 }
            },
            token
        );
    }

    #[test]
    fn it_parses_numbers_with_dot() {
        let input = "123.123".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Number(123.123),
                loc: SrcLocation { line: 1, col: 7 }
            },
            token
        );
    }

    #[test]
    fn it_parses_negative_numbers() {
        let input = "-123".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Number(-123.0),
                loc: SrcLocation { line: 1, col: 4 }
            },
            token
        );
    }

    #[test]
    fn it_stop_parsing_ids_on_whitespace() {
        let input = "someId other".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Id("someId".to_string()),
                loc: SrcLocation { line: 1, col: 6 }
            },
            token
        );
    }

    #[test]
    fn it_stop_parsing_ids_on_r_paren() {
        let input = "someId(other".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Id("someId".to_string()),
                loc: SrcLocation { line: 1, col: 6 }
            },
            token
        );
    }

    #[test]
    fn it_stop_parsing_ids_on_l_paren() {
        let input = "someId)other".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Id("someId".to_string()),
                loc: SrcLocation { line: 1, col: 6 }
            },
            token
        );
    }

    #[test]
    fn it_stop_parsing_ids_on_comment() {
        let input = "someId;other".to_string();

        let token = Tokenizer::new(&input)
            .next_token()
            .expect("failed to lex number");

        assert_eq!(
            Token {
                kind: TokenKind::Id("someId".to_string()),
                loc: SrcLocation { line: 1, col: 6 }
            },
            token
        );
    }
}
