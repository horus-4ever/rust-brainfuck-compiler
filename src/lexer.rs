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
        let mut result = vec![];
        let mut column = 0;
        let mut row = 0;
        for (_, chr) in self.chars {
            match chr {
                '\n' => { row += 1; column = 0; },
                _ => {
                    if let Some(tok) = Token::try_from(((column, row), chr)).ok() {
                        result.push(tok);
                    }
                    column += 1;
                }
            }
        }
        result
    }
}