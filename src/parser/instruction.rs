use std::fmt;

use crate::operand::{CellOperand, Operand, OperandParseError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Load(Operand),
    Store(CellOperand),
    Add(Operand),
    Sub(Operand),
    Mult(Operand),
    Div(Operand),
    Read(CellOperand),
    Write(Operand),
    Jump(String),
    Jgtz(String),
    Jzero(String),
    Halt,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InstructionParseError {
    #[error("Expected a label after keyword {0}, got nothing")]
    LabelNotFound(String),
    #[error("Keyword `{0}` is not a valid keyword")]
    InvalidKeyword(String),
    #[error(transparent)]
    OperandParseError(#[from] OperandParseError),
    #[error("Expected nothing, found `{0}`")]
    UnexpectedArgument(String),
}

fn parse_label(keyword: &str, s: Option<&str>) -> Result<String, InstructionParseError> {
    match s {
        Some(v) => Ok(v.to_owned()),
        None => Err(InstructionParseError::LabelNotFound(keyword.to_owned())),
    }
}

impl TryFrom<(&str, Option<&str>)> for Instruction {
    type Error = InstructionParseError;
    fn try_from((keyword, argument): (&str, Option<&str>)) -> Result<Self, Self::Error> {
        match keyword.to_lowercase().as_str() {
            "load" => Ok(Self::Load(Operand::try_from((argument, keyword))?)),
            "store" => Ok(Self::Store(CellOperand::try_from((argument, keyword))?)),
            "add" => Ok(Self::Add(Operand::try_from((argument, keyword))?)),
            "sub" => Ok(Self::Sub(Operand::try_from((argument, keyword))?)),
            "mult" => Ok(Self::Mult(Operand::try_from((argument, keyword))?)),
            "div" => Ok(Self::Div(Operand::try_from((argument, keyword))?)),
            "read" => Ok(Self::Read(CellOperand::try_from((argument, keyword))?)),
            "write" => Ok(Self::Write(Operand::try_from((argument, keyword))?)),
            "jump" => Ok(Self::Jump(parse_label(keyword, argument)?)),
            "jgtz" => Ok(Self::Jgtz(parse_label(keyword, argument)?)),
            "jzero" => Ok(Self::Jzero(parse_label(keyword, argument)?)),
            "halt" => {
                if let Some(v) = argument {
                    Err(InstructionParseError::UnexpectedArgument(v.to_owned()))
                } else {
                    Ok(Self::Halt)
                }
            }
            _ => Err(InstructionParseError::InvalidKeyword(keyword.to_owned())),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Load(op) => write!(f, "LOAD {}", op),
            Instruction::Store(cell_op) => write!(f, "STORE {}", cell_op),
            Instruction::Add(op) => write!(f, "ADD {}", op),
            Instruction::Sub(op) => write!(f, "SUB {}", op),
            Instruction::Mult(op) => write!(f, "MULT {}", op),
            Instruction::Div(op) => write!(f, "DIV {}", op),
            Instruction::Read(cell_op) => write!(f, "READ {}", cell_op),
            Instruction::Write(op) => write!(f, "WRITE {}", op),
            Instruction::Jump(label) => write!(f, "JUMP {}", label),
            Instruction::Jgtz(label) => write!(f, "JGTZ {}", label),
            Instruction::Jzero(label) => write!(f, "JZERO {}", label),
            Instruction::Halt => write!(f, "HALT"),
        }
    }
}
