use crate::token::{Token, TokenKind};
use crate::errors::{CompilerError, ErrorKind};
use std::convert::{From};

pub struct Parser {
    tokens: Vec<Token>
}

impl From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Self{ tokens }
    }
}

impl Parser {
    fn analyse(&self) -> Result<(), CompilerError> {
        let mut counter = 0;
        for token in &self.tokens {
            match (counter, &token.kind) {
                (counter, _) if counter < 0 => {
                    return Err(CompilerError{
                        kind: ErrorKind::UnexpectedToken(token.position, token.kind.clone())
                    })
                }
                (_, TokenKind::LSBracket) => counter += 1,
                (_, TokenKind::RSBracket) => counter -= 1,
                _ => ()
            }
        }
        if let 0 = counter {
            Ok(())
        } else {
            Err(CompilerError{ kind: ErrorKind::UnexpectedEOF })
        }
    }

    fn group(self) -> Vec<(Token, usize)> {
        let mut result: Vec<(Token, usize)> = vec![];
        let mut iterator = self.tokens.into_iter();
        let mut counter = 1;
        let mut previous: Token = iterator.next().unwrap();
        for token in iterator {
            if previous.kind == token.kind {
                counter += 1;
            } else {
                result.push((previous, counter));
                previous = token;
                counter = 1;
            }
        }
        result.push((previous, counter));
        result
    }

    pub fn parse(self) -> Result<Vec<(Token, usize)>, CompilerError>{
        self.analyse()?;
        Ok(self.group())
    }
}