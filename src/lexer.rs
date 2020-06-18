use crate::token::{Token};
use std::str::{CharIndices};
use std::convert::{TryFrom};

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: CharIndices<'a>) -> Self {
        Self { chars }
    }

    pub fn tokenize(self) -> Vec<Token> {
        self.chars
            .filter_map(|(pos, chr)| Token::try_from((pos, chr)).ok())
            .collect()
    }
}