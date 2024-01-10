use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Number(i32),             // =x
    ValueInCell(i32),        // x
    ValueOfValueInCell(i32), // ^x
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperandParseError {
    #[error("Operand {0} is not a valid operand")]
    InvalidOperand(String),
    #[error("Expected operand for keyword `{0}`, found nothing")]
    OperandNotFound(String),
}

macro_rules! parse_number {
    ($e:expr) => {
        $e.parse::<i32>().unwrap()
    };
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn is_operand_number(s: &str) -> bool {
    s.chars().nth(0).unwrap() == '=' && is_number(&s[1..])
}

fn is_operand_value_in_cell(s: &str) -> bool {
    is_number(s)
}

fn is_operand_value_of_value_in_cell(s: &str) -> bool {
    s.chars().nth(0).unwrap() == '^' && is_number(&s[1..])
}

impl FromStr for Operand {
    type Err = OperandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if is_operand_number(s) => Ok(Self::Number(parse_number!(s[1..]))),
            s if is_operand_value_in_cell(s) => Ok(Self::ValueInCell(parse_number!(s))),
            s if is_operand_value_of_value_in_cell(s) => {
                Ok(Self::ValueOfValueInCell(parse_number!(s[1..])))
            }
            _ => Err(OperandParseError::InvalidOperand(s.to_owned())),
        }
    }
}

impl TryFrom<(Option<&str>, &str)> for Operand {
    type Error = OperandParseError;
    fn try_from((s, keyword): (Option<&str>, &str)) -> Result<Self, Self::Error> {
        match s {
            Some(s) => Operand::from_str(s),
            None => Err(OperandParseError::OperandNotFound(keyword.to_owned())),
        }
    }
}
