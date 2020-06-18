use crate::assembly_parts::*;
use crate::token::{Token, TokenKind};
use crate::errors::{CompilerError, ErrorKind};
use strfmt::strfmt;
use std::collections::{HashMap};
use std::convert::{From};
use std::fs::{File};
use std::io::{Write};
use std::process::{Command};
use std::env;

pub struct Compiler {
    tokens: Vec<(Token, usize)>
}

impl From<Vec<(Token, usize)>> for Compiler {
    fn from(tokens: Vec<(Token, usize)>) -> Self {
        Self{ tokens }
    }
}

impl Compiler {
    pub fn compile(self) -> Result<(), CompilerError> {
        let mut stack: Vec<(usize, usize)> = vec![];
        let mut handle = File::create("temp.asm").unwrap();
        handle.write(String::from(
            format!("{}
            section .data\n{}\n
            section .bss\ninput_byte resb 2\n
            section .text\nglobal _start\n{}\n{}\n{}\n{}",
            _DEFINES,
            _MESSAGES,
            _PRINT_FUNCTION,
            _INPUT_FUNCTION,
            _ON_ERROR,
            _START_FUNCTION
        )).as_bytes()).unwrap();
        for (token, n) in self.tokens {
            let mut result = String::new();
            let mut map: HashMap<String, String> = HashMap::new();
            match token.kind {
                TokenKind::Plus => {
                    map.insert("number".to_string(), format!("{}", n));
                    result.push_str(&strfmt(_INC_MEMORY, &map).unwrap());
                }
                TokenKind::Minus => {
                    map.insert("number".to_string(), format!("{}", n));
                    result.push_str(&strfmt(_DEC_MEMORY, &map).unwrap());
                }
                TokenKind::LArrow => {
                    map.insert("number".to_string(), format!("{}", n));
                    result.push_str(&strfmt(_CHECK_ESI_DEC, &map).unwrap());
                }
                TokenKind::RArrow => {
                    map.insert("number".to_string(), format!("{}", n));
                    result.push_str(&strfmt(_CHECK_ESI_INC, &map).unwrap());
                }
                TokenKind::Dot => {
                    map.insert("number".to_string(), format!("{}", n));
                    result.push_str(&strfmt(_ON_PRINT, &map).unwrap());
                }
                TokenKind::Comma => {
                    map.insert("number".to_string(), format!("{}", n));
                    for _ in 0..n { result.push_str(&strfmt(_ON_INPUT, &map).unwrap()) }
                }
                TokenKind::LSBracket => {
                    for i in 0..n { stack.push((token.position, token.position + i)) }
                    map.insert("level".to_string(), format!("{}", token.position));
                    result.push_str(&strfmt(_LOOP_BEGIN, &map).unwrap());
                }
                TokenKind::RSBracket => {
                    for _ in 0..n {
                        let (loopname, name) = stack.pop().unwrap();
                        map.insert("loopname".to_string(), format!("{}", loopname));
                        map.insert("name".to_string(), format!("{}", name));
                        result.push_str(&strfmt(_LOOP_END, &map).unwrap());
                    }
                }
                _ => unreachable!()
            }
            handle.write(result.as_bytes()).unwrap();
        }
        handle.write(_CODE_END.as_bytes()).unwrap();
        // execute the build script
        let output = Command::new("sh")
            .arg(format!("{}/src/build.sh", env::current_dir().unwrap().to_str().unwrap()))
            .arg(env::current_dir().unwrap().to_str().unwrap())
            .output()?;
        if !output.status.success() {
            // println!("{:?}", String::from_utf8(output.stderr));
            Err(CompilerError{ kind: ErrorKind::BuildScriptFailure })
        } else {
            Ok(())
        }
    }
}