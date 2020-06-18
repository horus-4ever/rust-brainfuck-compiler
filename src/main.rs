mod token;
mod compiler;
mod lexer;
mod parser;
mod errors;
mod assembly_parts;

use lexer::{Lexer};
use parser::{Parser};
use compiler::{Compiler};

use std::env;
use std::io::{Read};
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let filename = match args.next() {
        Some(arg) => arg,
        None => { println!("Use : ./compiler <filename>"); return }
    };
    let mut handle = fs::File::open(filename).unwrap();
    let mut result = String::new();
    handle.read_to_string(&mut result).unwrap();
    // lexer
    let parser: Parser = Lexer::new(result.char_indices()).tokenize().into();
    let compiler: Compiler = parser.parse().unwrap().into();
    match compiler.compile() {
        Err(error) => eprintln!("{:?}", error),
        Ok(_) => println!("Compile successful")
    }
}
