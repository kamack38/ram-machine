use ram_machine::operand::*;
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
