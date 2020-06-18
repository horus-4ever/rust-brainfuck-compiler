mod token;
mod compiler;
mod lexer;

use lexer::{Lexer};

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
    handle.read_to_string(&mut result);
    // lexer
    let mut tokens = Lexer::new(result.char_indices()).tokenize();
    println!("{:?}", tokens);
}
