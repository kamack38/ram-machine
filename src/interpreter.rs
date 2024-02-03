use crate::parser::{
    instruction::Instruction,
    operand::{CellOperand, CellValue, ExpandError, Operand},
    ram_code::RamCode,
    Parser, ParserError,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct RamMachine {
    code: RamCode,
    tape: Vec<Option<CellValue>>,
    pointer: usize,
    input: Vec<CellValue>,
    input_pointer: usize,
    output: Vec<CellValue>,
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("Buffer could not be accessed, because its value was never set.")]
pub struct BufferError;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RamMachineError {
    #[error(transparent)]
    ExpandError(#[from] ExpandError),
    #[error(transparent)]
    BufferError(#[from] BufferError),
    #[error(transparent)]
    InputAccessError(#[from] InputAccessError),
    #[error(transparent)]
    JumpError(#[from] JumpError),
    #[error("Addition of `{0}` to `{1}` failed.")]
    AdditionFailed(CellValue, CellValue),
    #[error("Subtraction of `{0}` from `{1}` failed.")]
    SubtractionFailed(CellValue, CellValue),
    #[error("Multiplication by `{0}` of `{1}~ failed.")]
    MultiplicationFailed(CellValue, CellValue),
    #[error("Division by `{0}` of `{1}` failed.")]
    DivisionFailed(CellValue, CellValue),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InputAccessError {
    #[error("Input at index `{0}` not found.")]
    NotExistentInput(usize),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum JumpError {
    #[error("Label `{0}` could not be found in Ram code.")]
    LabelNotFound(String),
}

#[derive(PartialEq, Eq)]
pub enum RunState {
    Running,
    Halted,
}

impl RamMachine {
    pub fn new(code: RamCode, input: Vec<CellValue>) -> Self {
        RamMachine {
            code,
            tape: vec![None],
            pointer: 0,
            input_pointer: 0,
            input,
            output: Vec::new(),
        }
    }

    pub fn from_str(str: &str, input: Vec<CellValue>) -> Result<Self, ParserError> {
        Ok(RamMachine::new(Parser::new().parse(str)?, input))
    }

    pub fn run(mut self) -> Result<Vec<CellValue>, RamMachineError> {
        loop {
            let instruction = self.code.instructions[self.pointer].clone();
            if self.execute(&instruction)? == RunState::Halted {
                break;
            }
        }
        Ok(self.output)
    }

    // pub fn run_line(&mut self) -> Result<Vec<CellValue>, RamMachineError> {
    //     todo!()
    // }

    fn jump_to(&mut self, label: &str) -> Result<RunState, JumpError> {
        match self
            .code
            .jump_table
            .get(label)
            .ok_or_else(|| JumpError::LabelNotFound(label.to_owned()))
        {
            Ok(v) => {
                if *v < self.code.instructions.len() {
                    self.pointer = *v;
                    return Ok(RunState::Running);
                }
                Ok(RunState::Halted)
            }
            Err(e) => Err(e),
        }
    }

    fn get_input(&mut self) -> Result<&CellValue, InputAccessError> {
        let input = self
            .input
            .get(self.input_pointer)
            .ok_or(InputAccessError::NotExistentInput(self.input_pointer));
        self.input_pointer += 1;
        input
    }

    fn advance_pointer(&mut self) -> RunState {
        self.pointer += 1;
        if self.pointer < self.code.instructions.len() {
            return RunState::Running;
        }
        RunState::Halted
    }

    fn get(&self, operand: &Operand) -> Result<CellValue, ExpandError> {
        Ok(*operand.expand(&self.tape)?)
    }

    fn set(&mut self, cell_operand: &CellOperand, value: CellValue) -> Result<(), ExpandError> {
        let index = cell_operand.expand(&self.tape)?;
        if self.tape.len() < index + 1 {
            self.tape.resize(index + 1, None);
        }
        *self.tape.get_mut(index).expect("Tape was just resized") = Some(value);
        Ok(())
    }

    fn buffer(&self) -> Result<&CellValue, BufferError> {
        self.tape
            .first()
            .and_then(|val| val.as_ref())
            .ok_or(BufferError)
    }

    fn buffer_mut(&mut self) -> &mut Option<CellValue> {
        self.tape
            .get_mut(0)
            .expect("Tape was initialized with length 1")
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<RunState, RamMachineError> {
        use Instruction::*;
        match instruction {
            Load(o) => {
                *self.buffer_mut() = Some(self.get(o)?);
                Ok(self.advance_pointer())
            }
            Store(o) => {
                self.set(o, *self.buffer()?)?;
                Ok(self.advance_pointer())
            }
            Add(o) => {
                *self.buffer_mut() = Some(self.buffer()?.checked_add(self.get(o)?).ok_or(
                    RamMachineError::AdditionFailed(
                        self.get(o).expect("Checked before"),
                        *self.buffer().expect("Checked before"),
                    ),
                )?);
                Ok(self.advance_pointer())
            }
            Sub(o) => {
                *self.buffer_mut() = Some(self.buffer()?.checked_sub(self.get(o)?).ok_or(
                    RamMachineError::SubtractionFailed(
                        self.get(o).expect("Checked before"),
                        *self.buffer().expect("Checked before"),
                    ),
                )?);
                Ok(self.advance_pointer())
            }
            Mult(o) => {
                *self.buffer_mut() = Some(self.buffer()?.checked_mul(self.get(o)?).ok_or(
                    RamMachineError::MultiplicationFailed(
                        self.get(o).expect("Checked before"),
                        *self.buffer().expect("Checked before"),
                    ),
                )?);
                Ok(self.advance_pointer())
            }
            Div(o) => {
                *self.buffer_mut() = Some(self.buffer()?.checked_div(self.get(o)?).ok_or(
                    RamMachineError::DivisionFailed(
                        self.get(o).expect("Checked before"),
                        *self.buffer().expect("Checked before"),
                    ),
                )?);
                Ok(self.advance_pointer())
            }
            Read(o) => {
                let input = *self.get_input()?;
                self.set(o, input)?;
                Ok(self.advance_pointer())
            }
            Write(o) => {
                self.output.push(self.get(o)?);
                Ok(self.advance_pointer())
            }
            Jump(s) => Ok(self.jump_to(s)?),
            Jgtz(s) => {
                if *self.buffer()? > 0 {
                    self.jump_to(s)?;
                    return Ok(RunState::Running);
                }
                Ok(self.advance_pointer())
            }
            Jzero(s) => {
                if *self.buffer()? == 0 {
                    self.jump_to(s)?;
                    Ok(RunState::Running)
                } else {
                    Ok(self.advance_pointer())
                }
            }
            Halt => Ok(RunState::Halted),
        }
    }
}

#[cfg(test)]
mod interpreter_test {
    use crate::parser::Parser;

    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn three_sum() {
        let path = "./examples/three_sum.ram";
        let file = read_to_string(path).unwrap();
        let code = Parser::new().parse(&file).unwrap();
        let input1: Vec<i64> = vec![1, 3, 2];
        let input2: Vec<i64> = vec![-1232323, 34324, 92384];
        let input3: Vec<i64> = vec![324, 546, 8023];
        let input4: Vec<i64> = vec![3209847, 16879823, 27034];
        let input5: Vec<i64> = vec![0, 0, 0];
        assert_eq!(
            RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
            vec![input1.iter().sum()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
            vec![input2.iter().sum()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
            vec![input3.iter().sum()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
            vec![input4.iter().sum()]
        );
        assert_eq!(
            RamMachine::new(code, input5.clone()).run().unwrap(),
            vec![input5.iter().sum()]
        );
    }

    #[test]
    fn square() {
        let path = "./examples/square.ram";
        let file = read_to_string(path).unwrap();
        let code = Parser::new().parse(&file).unwrap();
        let input1 = vec![36];
        let input2 = vec![0];
        let input3 = vec![1_000_000_000];
        let input4 = vec![978314014];
        let input5 = vec![32423];
        assert_eq!(
            RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
            vec![input1[0] * input1[0]]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
            vec![input2[0] * input2[0]]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
            vec![input3[0] * input3[0]]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
            vec![input4[0] * input4[0]]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
            vec![input5[0] * input5[0]]
        );
    }

    #[test]
    fn sequence_length() {
        let path = "./examples/sequence_length.ram";
        let file = read_to_string(path).unwrap();
        let code = Parser::new().parse(&file).unwrap();
        let input1 = vec![1, 2, 4, 6, 8, 9, 10, 0];
        let input2 = vec![1, 5, 6, 7, 0];
        let mut input3 = vec![66; 66];
        input3.push(0);
        let input4 = vec![0];
        let mut input5 = vec![40; 1_000_000];
        input5.push(0);
        assert_eq!(
            RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
            vec![(input1.len() - 1).try_into().unwrap()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
            vec![(input2.len() - 1).try_into().unwrap()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
            vec![(input3.len() - 1).try_into().unwrap()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
            vec![(input4.len() - 1).try_into().unwrap()]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
            vec![(input5.len() - 1).try_into().unwrap()]
        );
    }

    #[test]
    fn log() {
        let path = "./examples/log.ram";
        let file = read_to_string(path).unwrap();
        let code = Parser::new().parse(&file).unwrap();
        let input1 = vec![2, 2 << 31];
        let input2 = vec![5, 1];
        let input3 = vec![3, 55];
        let input4 = vec![3, 999_999_999_999_999_999];
        let input5 = vec![40, 1_000_000_000];
        assert_eq!(
            RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
            vec![32]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
            vec![0]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
            vec![3]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
            vec![37]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
            vec![5]
        );
    }

    #[test]
    fn unit_digit() {
        let path = "./examples/unit_digit.ram";
        let file = read_to_string(path).unwrap();
        let code = Parser::new().parse(&file).unwrap();
        let input1 = vec![320423789];
        let input2 = vec![-234234235];
        let input3 = vec![999_999_999_999_999_991];
        let input4 = vec![0];
        let input5 = vec![576];
        assert_eq!(
            RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
            vec![9]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
            vec![5]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
            vec![1]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
            vec![0]
        );
        assert_eq!(
            RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
            vec![6]
        );
    }
}
