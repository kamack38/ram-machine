use std::fs;

use crate::parser::Parser;

mod instruction;
mod operand;
mod parser;
mod ram_code;

fn main() -> Result<(), parser::ParserError> {
    let unparsed_file = fs::read_to_string("code.ram").unwrap();
    // let unparsed_file = "store";
    let parser = Parser::new();
    let code = parser.parse(&unparsed_file)?;
    println!("{:?}", code);
    Ok(())
}
