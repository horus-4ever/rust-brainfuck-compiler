use crate::token::{Token, TokenKind};
use std::str::{CharIndices};
use std::convert::{TryFrom};

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: CharIndices<'a>) -> Self {
        Self { chars }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        self.chars
            .filter_map(|(pos, chr)| Token::try_from((pos, chr)).ok())
            .collect()
    }
}