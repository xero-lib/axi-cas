use crate::Token;

pub struct Lexer<'a> {
    data: &'a str,
    cursor: usize,
    next: Token<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data, cursor: 0, next: Token::Start }
    }
}