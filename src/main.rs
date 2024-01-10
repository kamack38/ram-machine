use std::fs;

use crate::parser::parser::{Parser, ParserError};

mod parser;

fn main() -> Result<(), ParserError> {
    let unparsed_file = fs::read_to_string("code.ram").unwrap();
    // let unparsed_file = "store";
    let parser = Parser::new();
    let code = parser.parse(&unparsed_file)?;
    println!("{:?}", code);
    Ok(())
}
