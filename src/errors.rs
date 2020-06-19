use std::error::{Error};
use std::fmt;
use crate::token::{TokenKind};

#[derive(fmt::Debug)]
pub struct CompilerError {
    pub kind: ErrorKind,
}

#[derive(fmt::Debug)]
pub enum ErrorKind {
    UnexpectedToken((usize, usize), TokenKind),
    UnexpectedEOF,
    BuildScriptFailure,
    IO(std::io::Error)
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Parser Error : <{:?}>", self.kind)
    }
}

impl Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        Self{ kind: ErrorKind::IO(error) }
    }
}