use std::fmt::{Debug};
use std::convert::{TryFrom};
use std::clone::{Clone};
use std::cmp::{PartialEq};

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub position: (usize, usize)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    RArrow,
    LArrow,
    RSBracket,
    LSBracket,
    Dot,
    Comma
}

impl TryFrom<char> for TokenKind {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '<' => Ok(Self::LArrow),
            '>' => Ok(Self::RArrow),
            '[' => Ok(Self::LSBracket),
            ']' => Ok(Self::RSBracket),
            '.' => Ok(Self::Dot),
            ',' => Ok(Self::Comma),
            _ => Err(())
        }
    }
}

impl TryFrom<((usize, usize), char)> for Token {
    type Error = ();
    fn try_from(value: ((usize, usize), char)) -> Result<Self, Self::Error> {
        let kind = TokenKind::try_from(value.1)?;
        Ok(Self { kind, position: value.0 })
    }
}