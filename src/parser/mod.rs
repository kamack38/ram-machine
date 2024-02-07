pub mod instruction;
pub mod operand;
pub mod ram_code;

use instruction::{Instruction, InstructionParseError};
use ram_code::RamCode;
use thiserror::Error;

pub struct Parser {}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParserError {
    #[error("Expected EOL, found `{0}`")]
    UnexpectedArgument(String),
    #[error(transparent)]
    InstructionParseError(#[from] InstructionParseError),
}

macro_rules! return_if_comment {
    ($e:expr) => {
        if $e.starts_with('#') {
            return Ok(());
        }
    };
}

impl Parser {
    const LABEL_END: char = ':';

    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(self, string: &str) -> Result<RamCode, ParserError> {
        let mut code = RamCode::new();
        let lines = string.lines().filter(|s| !s.is_empty());
        for line in lines {
            self.parse_line(line, &mut code)?;
        }
        Ok(code)
    }

    pub fn parse_line(&self, line: &str, code: &mut RamCode) -> Result<(), ParserError> {
        let mut slices = line.split_whitespace().filter(|s| !s.is_empty());

        let mut slice = match slices.next() {
            None => return Ok(()),
            Some(val) => val,
        };

        return_if_comment!(slice);

        if slice.ends_with(Self::LABEL_END) {
            code.jump_table.insert(
                slice.trim_end_matches(':').to_owned(),
                code.instructions.len(),
            );

            slice = match slices.next() {
                Some(val) => val,
                None => return Ok(()),
            }
        }

        return_if_comment!(slice);

        let argument = slices.next();

        let instruction = Instruction::try_from((slice, argument))?;
        code.add_instruction(instruction);

        let rest = slices.next();

        if let Some(v) = rest {
            return_if_comment!(v);
            return Err(ParserError::UnexpectedArgument(v.to_string()));
        }

        Ok(())
    }
}
