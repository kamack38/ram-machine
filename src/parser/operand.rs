use std::str::FromStr;
use thiserror::Error;

pub type CellAddress = usize;
pub type CellValue = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Number(CellValue),               // =x
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
    #[error("Value `{0}` in cell `{1}` could not be converted to a tape index.")]
    ConvertError(CellValue, CellAddress),
    #[error("Tried reading from cell with address `{0}`, which was never set.")]
    ValueNotSet(CellAddress),
}

impl Operand {
    pub fn expand<'a>(
        &'a self,
        tape: &'a [Option<CellValue>],
    ) -> Result<&'a CellValue, ExpandError> {
        use Operand::*;
        match self {
            Number(v) => Ok(v),
            ValueInCell(cell) => Ok(tape
                .get(*cell)
                .and_then(|val| val.as_ref())
                .ok_or(ExpandError::ValueNotSet(*cell))?),
            ValueOfValueInCell(cell) => tape
                .get(
                    CellAddress::try_from(
                        tape.get(*cell)
                            .ok_or(ExpandError::ValueNotSet(*cell))?
                            .ok_or(ExpandError::ValueNotSet(*cell))?,
                    )
                    .map_err(|_| {
                        ExpandError::ConvertError(
                            tape.get(*cell).expect("Would've failed before").unwrap(),
                            *cell,
                        )
                    })?,
                )
                .and_then(|val| val.as_ref())
                .ok_or_else(|| {
                    ExpandError::ValueNotSet(
                        CellAddress::try_from(
                            tape.get(*cell).expect("Would've failed before").unwrap(),
                        )
                        .expect("Would've failed before"),
                    )
                }),
        }
    }
}

impl CellOperand {
    pub fn expand(&self, tape: &[Option<CellValue>]) -> Result<CellAddress, ExpandError> {
        use CellOperand::*;
        match self {
            AddressOfCell(cell) => Ok(*cell),
            AddressOfCellInCell(cell) => CellAddress::try_from(
                tape.get(*cell)
                    .ok_or(ExpandError::ValueNotSet(*cell))?
                    .ok_or(ExpandError::ValueNotSet(*cell))?,
            )
            .map_err(|_| {
                ExpandError::ConvertError(
                    tape.get(*cell).expect("Didn't fail previously").unwrap(),
                    *cell,
                )
            }),
        }
    }
}

macro_rules! parse {
    ($e:expr) => {
        $e.parse().expect("Already checked if can be parsed")
    };
}

fn is_number(s: &str) -> bool {
    s.parse::<CellValue>().is_ok()
}

fn is_positive_number(s: &str) -> bool {
    s.parse::<CellAddress>().is_ok()
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

#[cfg(test)]
mod operand_tests {
    use super::*;
    use CellOperand::*;
    use Operand::*;

    #[test]
    fn expand_address_of_cell() {
        let o = AddressOfCell(2);
        assert_eq!(o.expand(&vec![Some(0), None, Some(5)]), Ok(2));
    }

    #[test]
    fn expand_address_of_cell_in_cell() {
        let o = AddressOfCellInCell(6);
        let mut tape = vec![None; 6];
        tape.push(Some(7));
        tape.push(Some(2));
        assert_eq!(o.expand(&tape), Ok(7));
    }

    #[test]
    fn expand_address_of_cell_in_cell_fail() {
        let o = AddressOfCellInCell(10);
        let mut tape = vec![None; 10];
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(10)));
        tape.push(None);
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(10)));
        tape.pop();
        tape.push(Some(-5));
        assert_eq!(o.expand(&tape), Err(ExpandError::ConvertError(-5, 10)));
    }

    #[test]
    fn expand_number() {
        let o = Number(10000000);
        assert_eq!(o.expand(&vec![]), Ok(&10000000));
    }

    #[test]
    fn expand_value_in_cell() {
        let o = ValueInCell(8);
        let mut tape = vec![None; 8];
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(8)));
        tape.push(Some(20));
        assert_eq!(o.expand(&tape), Ok(&20));
    }

    #[test]
    fn expand_value_of_value_in_cell() {
        let o = ValueOfValueInCell(20);
        let mut tape = vec![None; 20];
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(20)));
        tape.push(None);
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(20)));
        tape[20] = Some(-500);
        assert_eq!(o.expand(&tape), Err(ExpandError::ConvertError(-500, 20)));
        tape[20] = Some(16);
        assert_eq!(o.expand(&tape), Err(ExpandError::ValueNotSet(16)));
        tape[16] = Some(8);
        assert_eq!(o.expand(&tape), Ok(&8));
    }
}
