use std::fmt::{Debug};
use std::convert::{TryFrom};

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    position: usize
}

#[derive(Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Rarrow,
    LArrow,
    RSBracket,
    LSBracket,
    Dot,
    Comma,
    EOF
}

impl TryFrom<char> for TokenKind {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '<' => Ok(Self::LArrow),
            '>' => Ok(Self::Rarrow),
            '[' => Ok(Self::LSBracket),
            ']' => Ok(Self::RSBracket),
            '.' => Ok(Self::Dot),
            ',' => Ok(Self::Comma),
            _ => Err(())
        }
    }
}

impl TryFrom<(usize, char)> for Token {
    type Error = ();
    fn try_from(value: (usize, char)) -> Result<Self, Self::Error> {
        let kind = TokenKind::try_from(value.1)?;
        Ok(Self { kind, position: value.0 })
    }
}