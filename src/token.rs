use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(usize),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Slash,
    Equal,
    NotEqual,
    Lt,
    Gt,
    Le,
    Ge,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    If,
    Else,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "fn" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    // add code here
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        loop {
            match self.read_char() {
                Some(',') => return Some(Token::Comma),
                Some(';') => return Some(Token::Semicolon),
                Some('(') => return Some(Token::LParen),
                Some(')') => return Some(Token::RParen),
                Some('{') => return Some(Token::LBrace),
                Some('}') => return Some(Token::RBrace),
                Some('/') => return Some(Token::Slash),
                Some('+') => return Some(Token::Plus),
                Some('-') => return Some(Token::Minus),
                Some('!') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::NotEqual);
                        }
                    }
                    return Some(Token::Bang);
                }
                Some('=') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Equal);
                        }
                    }
                    return Some(Token::Assign);
                }
                Some('>') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Ge);
                        }
                    }
                    return Some(Token::Gt);
                }
                Some('<') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Le);
                        }
                    }
                    return Some(Token::Lt);
                }
                Some(c) if c.is_ascii_digit() => {
                    let str = self.keep_reading(c, |c| c.is_ascii_digit());
                    let str = str.into_iter().collect::<String>();
                    return Some(Token::Int(str::parse::<usize>(&str).expect("aint no way")));
                }
                Some(c) if c.is_ascii_alphabetic() => {
                    let str = self.keep_reading(c, |c| c.is_ascii_alphabetic());
                    let str = str.into_iter().collect::<String>();

                    if let Some((_, keyword)) = KEYWORDS.get_entry(&str) {
                        return Some(keyword.clone());
                    }

                    return Some(Token::Ident(str));
                }
                Some(_) => return Some(Token::Illegal),
                _ => return None,
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Lexer<'a> {
        return Lexer {
            chars: code.chars().peekable(),
        };
    }

    fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }

    fn read_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn skip_whitespace(&mut self) {
        while self.chars.next_if(|c| c.is_whitespace()).is_some() {}
    }

    fn keep_reading(&mut self, c: char, f: impl Fn(&char) -> bool) -> Vec<char> {
        let mut out = vec![c];
        while let Some(val) = self.chars.next_if(&f) {
            out.push(val)
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_iterator() {
        let input = "=+(){},;";
        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input);
        let result = lexer.into_iter().collect::<Vec<Token>>();
        assert_eq!(result, expected_tokens);
    }
}
