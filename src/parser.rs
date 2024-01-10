use crate::instruction::{Instruction, InstructionParseError};
use crate::ram_code::RAMCode;
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

    pub fn parse(self, string: &str) -> Result<RAMCode, ParserError> {
        let mut code = RAMCode::new();
        let lines = string.lines().filter(|s| !s.is_empty());
        for line in lines {
            self.parse_line(line, &mut code)?;
        }
        Ok(code)
    }

    pub fn parse_line(&self, line: &str, code: &mut RAMCode) -> Result<(), ParserError> {
        let mut slices = line.split(' ').filter(|s| !s.is_empty());

        let mut slice = match slices.next() {
            None => return Ok(()),
            Some(val) => val,
        };

        return_if_comment!(slice);

        if slice.ends_with(Self::LABEL_END) {
            code.jump_table.insert(
                slice.trim_end_matches(':').to_owned(),
                u32::try_from(code.instructions.len()).unwrap(),
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

        if rest.is_some() {
            return_if_comment!(rest.expect("Tested if is some"));
            return Err(ParserError::UnexpectedArgument(rest.unwrap().to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::operand::Operand;
    use std::collections::HashMap;
    use Instruction::Add as A;
    use Instruction::Div as D;
    use Instruction::Halt as H;
    use Instruction::Jgtz;
    use Instruction::Jump as Jmp;
    use Instruction::Jzero as Jz;
    use Instruction::Load as L;
    use Instruction::Mult as M;
    use Instruction::Read as R;
    use Instruction::Store as S;
    use Instruction::Write as W;
    use Operand::Number as Num;
    use Operand::ValueInCell as VC;
    use Operand::ValueOfValueInCell as VOC;

    #[test]
    fn parse_countdown() {
        let code = "
load =10
write 0
loop: add =-1
write 0
jzero halt
jump loop
halt: halt
";

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![
                L(Num(10)),
                W(VC(0)),
                A(Num(-1)),
                W(VC(0)),
                Jz(String::from("halt")),
                Jmp(String::from("loop")),
                H,
            ],
            jump_table: HashMap::from([("loop".to_owned(), 2), ("halt".to_owned(), 6)]),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_loop() {
        let code = "
# this is a comment
read 1
label: # great loop
load 1
add =-1
store 1
add =3
read ^0
load 1
jgtz label
halt
";

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![
                R(VC(1)),
                L(VC(1)),
                A(Num(-1)),
                S(VC(1)),
                A(Num(3)),
                R(VOC(0)),
                L(VC(1)),
                Jgtz(String::from("label")),
                H,
            ],
            jump_table: HashMap::from([("label".to_owned(), 1)]),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_negative_numbers() {
        let code = "
load =-3
rEad 1
add 1
mult =-2
div =-5
";

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![L(Num(-3)), R(VC(1)), A(VC(1)), M(Num(-2)), D(Num(-5))],
            jump_table: HashMap::new(),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_instructions_with_operands() {
        let code = "
read 1
load 3
store ^4
write 3
mult =2
div ^4
halt
";

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![
                R(VC(1)),
                L(VC(3)),
                S(VOC(4)),
                W(VC(3)),
                M(Num(2)),
                D(VOC(4)),
                H,
            ],
            jump_table: HashMap::new(),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_comment() {
        let code = "
read 1
load 1
# this is a comment
muLt =2
add =5
";

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![R(VC(1)), L(VC(1)), M(Num(2)), A(Num(5))],
            jump_table: HashMap::new(),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_label_and_comment() {
        let label = "test_label";
        let code = format!("{}: # this is a comment", label);

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![],
            jump_table: HashMap::from([(label.to_owned(), 0)]),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_double_label() {
        let label = "test_label";
        let code = format!("{}: {}2:", label, label);

        let parser = Parser::new();

        assert_eq!(
            parser.parse(&code),
            Err(ParserError::InstructionParseError(
                InstructionParseError::InvalidKeyword(format!("{}2:", label))
            ))
        );
    }

    #[test]
    fn parse_label() {
        let label = "test_label";
        let code = format!(
            "
Load =4
{}: stOrE 3
",
            label
        );

        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![L(Num(4)), S(VC(3))],
            jump_table: HashMap::from([(label.to_owned(), 1)]),
        };
        assert_eq!(parser.parse(&code), Ok(expected_code));
    }

    #[test]
    fn parse_load() {
        let code = "
lOaD 1
";
        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![L(VC(1))],
            jump_table: HashMap::new(),
        };
        assert_eq!(parser.parse(code), Ok(expected_code));
    }

    #[test]
    fn parse_double_instruction() {
        let code = "
read 1
load 1 add =3
store 1
";
        let parser = Parser::new();

        assert_eq!(
            parser.parse(code),
            Err(ParserError::UnexpectedArgument("add".to_string()))
        );
    }

    #[test]
    fn parse_comment_after_instruction() {
        let code = "
read 1
load 1 # this is a comment
add =3
store 1
";
        let parser = Parser::new();

        let expected_code = RAMCode {
            instructions: vec![R(VC(1)), L(VC(1)), A(Num(3)), S(VC(1))],
            jump_table: HashMap::new(),
        };

        assert_eq!(parser.parse(code), Ok(expected_code));
    }
}
