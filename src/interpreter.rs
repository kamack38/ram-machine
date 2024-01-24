use crate::parser::{
    instruction::Instruction,
    operand::{CellOperand, ExpandError, Operand},
    ram_code::RamCode,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct RamMachine {
    code: RamCode,
    tape: Vec<Option<i32>>,
    pointer: usize,
    input: Vec<i32>,
    input_pointer: usize,
    output: Vec<i32>,
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
    #[error("Addition of `{0}` failed.")]
    AdditionFailed(i32),
    #[error("Multiplication by `{0}` failed.")]
    MultiplicationFailed(i32),
    #[error("Division by `{0}` failed.")]
    DivisionFailed(i32),
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
    pub fn new(code: RamCode, input: Vec<i32>) -> Self {
        RamMachine {
            code,
            tape: vec![None],
            pointer: 0,
            input_pointer: 0,
            input,
            output: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Vec<i32>, RamMachineError> {
        loop {
            let instruction = self.code.instructions[self.pointer].clone();
            if self.execute(&instruction)? == RunState::Halted {
                break;
            }
        }
        Ok(self.output)
    }

    // pub fn run_line(&mut self) -> Result<Vec<i32>, RamMachineError> {
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

    fn get_input(&mut self) -> Result<&i32, InputAccessError> {
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

    fn get(&self, operand: &Operand) -> Result<i32, ExpandError> {
        Ok(*operand.expand(&self.tape)?)
    }

    fn set(&mut self, cell_operand: &CellOperand, value: i32) -> Result<(), ExpandError> {
        let index = cell_operand.expand(&self.tape)?;
        if self.tape.len() < index + 1 {
            self.tape.resize(index + 1, None);
        }
        *self.tape.get_mut(index).expect("Tape was just resized") = Some(value);
        Ok(())
    }

    fn buffer(&self) -> Result<&i32, BufferError> {
        self.tape
            .first()
            .and_then(|val| val.as_ref())
            .ok_or(BufferError)
    }

    fn buffer_mut(&mut self) -> &mut Option<i32> {
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
                *self.buffer_mut() = Some(
                    self.buffer()?
                        .checked_add(self.get(o)?)
                        .ok_or(RamMachineError::AdditionFailed(self.get(o)?))?,
                );
                Ok(self.advance_pointer())
            }
            Mult(o) => {
                *self.buffer_mut() = Some(
                    self.buffer()?
                        .checked_mul(self.get(o)?)
                        .ok_or(RamMachineError::MultiplicationFailed(self.get(o)?))?,
                );
                Ok(self.advance_pointer())
            }
            Div(o) => {
                *self.buffer_mut() = Some(
                    self.buffer()?
                        .checked_div(self.get(o)?)
                        .ok_or(RamMachineError::DivisionFailed(self.get(o)?))?,
                );
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
