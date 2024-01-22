use std::env;
use std::fs;
use std::num::ParseIntError;
use std::process::exit;
use thiserror::Error;

use crate::interpreter::{RamMachine, RamMachineError};
use crate::parser::{Parser, ParserError};

mod interpreter;
mod parser;

#[derive(Error, Debug, PartialEq, Eq)]
enum RuntimeError {
    #[error("Too few args given")]
    NotEnoughArgs,
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error(transparent)]
    InterpreterError(#[from] RamMachineError),
    #[error(transparent)]
    ConvertInputError(#[from] ParseIntError),
}

fn main() -> Result<(), RuntimeError> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }
    let mut input: Vec<i32> = Vec::new();
    for arg in &args[2..] {
        input.push(arg.parse()?);
    }
    let unparsed_file = fs::read_to_string(args[1].as_str()).unwrap();
    let parser = Parser::new();
    let code = match parser.parse(&unparsed_file) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    let interpreter = RamMachine::new(code, input);
    match interpreter.run() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
    Ok(())
}
