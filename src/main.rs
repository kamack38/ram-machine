use std::env;
use std::fs;
use std::num::ParseIntError;
use std::process::exit;
use thiserror::Error;

use crate::interpreter::RamMachine;

mod interpreter;
mod parser;

#[derive(Error, Debug, PartialEq, Eq)]
enum RuntimeError {
    #[error("Too few args given")]
    NotEnoughArgs,
    #[error(transparent)]
    ConvertInputError(#[from] ParseIntError),
}

fn main() -> Result<(), RuntimeError> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }
    let mut input: Vec<i64> = Vec::new();
    for arg in &args[2..] {
        input.push(arg.parse()?);
    }
    let unparsed_file = fs::read_to_string(args[1].as_str()).unwrap();
    let interpreter = match RamMachine::from_str(&unparsed_file, input) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    match interpreter.run() {
        Ok(v) => println!("{:?}", v),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
    Ok(())
}
