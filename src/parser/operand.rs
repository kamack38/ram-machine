use std::str::FromStr;
use thiserror::Error;

pub type CellAddress = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Number(i32),                     // =x
    ValueInCell(CellAddress),        // x
    ValueOfValueInCell(CellAddress), // ^x
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum OperandParseError {
    #[error("Operand {0} is not a valid operand")]
    InvalidOperand(String),
    #[error("Expected operand for keyword `{0}`, found nothing")]
    OperandNotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellOperand {
    AddressOfCell(CellAddress),
    AddressOfCellInCell(CellAddress),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExpandError {
    #[error("Cell {0} does not exist or its value wasn't set.")]
    NotExistentCell(usize),
    #[error("Value `{0}` in cell `{1}` could not be converted to a tape index.")]
    ConvertError(i32, usize),
    // #[error("Tried reading from cell with address `{0}`, which wasn't set.")]
    // ValueNotSet(usize),
}

impl Operand {
    pub fn expand<'a>(&'a self, tape: &'a [i32]) -> Result<&'a i32, ExpandError> {
        use Operand::*;
        match self {
            Number(v) => Ok(v),
            ValueInCell(cell) => tape.get(*cell).ok_or(ExpandError::NotExistentCell(*cell)),
            ValueOfValueInCell(cell) => tape
                .get(
                    CellAddress::try_from(
                        *tape.get(*cell).ok_or(ExpandError::NotExistentCell(*cell))?,
                    )
                    .or(Err(ExpandError::ConvertError(
                        *tape.get(*cell).expect("Would've failed before"),
                        *cell,
                    )))?,
                )
                .ok_or(ExpandError::NotExistentCell(
                    CellAddress::try_from(*tape.get(*cell).expect("Would've failed before"))
                        .expect("Would've failed before"),
                )),
        }
    }
}

impl CellOperand {
    pub fn expand(&self, tape: &[i32]) -> Result<CellAddress, ExpandError> {
        use CellOperand::*;
        match self {
            AddressOfCell(cell) => Ok(*cell),
            AddressOfCellInCell(cell) => {
                CellAddress::try_from(*tape.get(*cell).ok_or(ExpandError::NotExistentCell(*cell))?)
                    .or(Err(ExpandError::ConvertError(
                        *tape.get(*cell).expect("Didn't fail previously"),
                        *cell,
                    )))
            }
        }
    }
}

macro_rules! parse {
    ($e:expr) => {
        $e.parse().expect("Already checked if can be parsed")
    };
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn is_positive_number(s: &str) -> bool {
    // println!("ok");
    s.parse::<usize>().is_ok()
}

fn is_operand_number(s: &str) -> bool {
    s.starts_with('=') && is_number(&s[1..])
}

fn is_operand_value_in_cell(s: &str) -> bool {
    is_positive_number(s)
}

fn is_operand_value_of_value_in_cell(s: &str) -> bool {
    s.starts_with('^') && is_positive_number(&s[1..])
}
fn is_operand_address_of_cell(s: &str) -> bool {
    is_operand_value_in_cell(s)
}

fn is_operand_address_of_cell_in_cell(s: &str) -> bool {
    is_operand_value_of_value_in_cell(s)
}

impl FromStr for Operand {
    type Err = OperandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if is_operand_number(s) => Ok(Self::Number(parse!(s[1..]))),
            s if is_operand_value_in_cell(s) => Ok(Self::ValueInCell(parse!(s))),
            s if is_operand_value_of_value_in_cell(s) => {
                Ok(Self::ValueOfValueInCell(parse!(s[1..])))
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

impl FromStr for CellOperand {
    type Err = OperandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if is_operand_address_of_cell(s) => Ok(Self::AddressOfCell(parse!(s))),
            s if is_operand_address_of_cell_in_cell(s) => {
                Ok(Self::AddressOfCellInCell(parse!(s[1..])))
            }
            _ => Err(OperandParseError::InvalidOperand(s.to_owned())),
        }
    }
}

impl TryFrom<(Option<&str>, &str)> for CellOperand {
    type Error = OperandParseError;
    fn try_from((s, keyword): (Option<&str>, &str)) -> Result<Self, Self::Error> {
        match s {
            Some(s) => CellOperand::from_str(s),
            None => Err(OperandParseError::OperandNotFound(keyword.to_owned())),
        }
    }
}
