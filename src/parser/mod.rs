pub mod instruction;
pub mod operand;

use crate::{instruction::Instruction, instruction::InstructionParseError};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

use super::operand::CellAddress;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RamCode {
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<String, CellAddress>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CodeParseError {
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

const LABEL_END: char = ':';

impl RamCode {
    pub fn new() -> RamCode {
        RamCode {
            instructions: Vec::new(),
            jump_table: HashMap::new(),
        }
    }

    pub fn push_line(&mut self, line: &str) -> Result<(), CodeParseError> {
        let mut slices = line.split_whitespace().filter(|s| !s.is_empty());

        let mut slice = match slices.next() {
            None => return Ok(()),
            Some(val) => val,
        };

        return_if_comment!(slice);

        if slice.ends_with(LABEL_END) {
            self.jump_table.insert(
                slice.trim_end_matches(':').to_owned(),
                self.instructions.len(),
            );

            slice = match slices.next() {
                Some(val) => val,
                None => return Ok(()),
            }
        }

        return_if_comment!(slice);

        let argument = slices.next();

        let instruction = Instruction::try_from((slice, argument))?;
        self.add_instruction(instruction);

        let rest = slices.next();

        if let Some(v) = rest {
            return_if_comment!(v);
            return Err(CodeParseError::UnexpectedArgument(v.to_string()));
        }

        Ok(())
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}

impl FromStr for RamCode {
    type Err = CodeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = RamCode::new();
        let lines = s.lines().filter(|line| !line.is_empty());
        for line in lines {
            code.push_line(line)?;
        }
        Ok(code)
    }
}
