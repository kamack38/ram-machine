use std::num::TryFromIntError;

use crate::parser::{
    instruction::Instruction,
    operand::CellAddress,
    operand::{CellOperand, Operand},
    ram_code::RamCode,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct RamMachine {
    code: RamCode,
    tape: Vec<i32>,
    pointer: usize,
    input: Vec<i32>,
    input_pointer: usize,
    output: Vec<i32>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RamMachineError {
    #[error(transparent)]
    CellAccessError(#[from] CellAccessError),
    #[error(transparent)]
    InputAccessError(#[from] InputAccessError),
    #[error(transparent)]
    ConvertError(#[from] TryFromIntError),
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
pub enum CellAccessError {
    #[error("Cell {0} does not exist or its value wasn't set.")]
    NotExistentCell(usize),
    #[error(transparent)]
    ConvertError(#[from] TryFromIntError),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InputAccessError {
    #[error("Input at index `{0}` not found.")]
    NotExistentInput(usize),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum JumpError {
    #[error("")]
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
            tape: vec![0, 0, 0, 0, 0, 0],
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

    fn access_cell(&self, cell: &CellAddress) -> Result<i32, CellAccessError> {
        self.tape
            .get(*cell)
            .ok_or(CellAccessError::NotExistentCell(*cell))
            .copied()
    }

    fn access(&self, operand: &Operand) -> Result<i32, CellAccessError> {
        use Operand::*;
        match operand {
            Number(v) => Ok(*v),
            ValueInCell(cell) => Ok(self.access_cell(cell)?),
            ValueOfValueInCell(cell) => {
                Ok(self.access_cell(&usize::try_from(self.access_cell(cell)?)?)?)
            }
        }
    }

    fn get_cell(&mut self, cell_operand: &CellOperand) -> Result<&mut i32, CellAccessError> {
        use CellOperand::*;
        match cell_operand {
            AddressOfCell(cell) => self
                .tape
                .get_mut(*cell)
                .ok_or(CellAccessError::NotExistentCell(*cell)),
            AddressOfCellInCell(cell) => {
                let real_cell = self
                    .tape
                    .get(*cell)
                    .ok_or(CellAccessError::NotExistentCell(*cell))?;
                self.get_cell(&CellOperand::AddressOfCell(usize::try_from(*real_cell)?))
            }
        }
    }

    fn jump_to(&mut self, label: &str) -> Result<RunState, JumpError> {
        match self
            .code
            .jump_table
            .get(label)
            .ok_or(JumpError::LabelNotFound(label.to_owned()))
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

    fn buffer_value(&self) -> i32 {
        self.tape[0]
    }

    fn buffer(&mut self) -> &mut i32 {
        self.tape.get_mut(0).unwrap()
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

    pub fn execute(&mut self, instruction: &Instruction) -> Result<RunState, RamMachineError> {
        use Instruction::*;
        match instruction {
            Load(o) => {
                self.tape[0] = self.access(o)?;
                Ok(self.advance_pointer())
            }
            Store(o) => {
                *self.get_cell(o)? = self.buffer_value();
                Ok(self.advance_pointer())
            }
            Add(o) => {
                *self.buffer() = self.tape[0]
                    .checked_add(self.access(o)?)
                    .ok_or(RamMachineError::AdditionFailed(self.access(o)?))?;
                Ok(self.advance_pointer())
            }
            Mult(o) => {
                *self.buffer() = self
                    .buffer_value()
                    .checked_mul(self.access(o)?)
                    .ok_or(RamMachineError::MultiplicationFailed(self.access(o)?))?;
                Ok(self.advance_pointer())
            }
            Div(o) => {
                *self.buffer() = self
                    .buffer_value()
                    .checked_div(self.access(o)?)
                    .ok_or(RamMachineError::DivisionFailed(self.access(o)?))?;
                Ok(self.advance_pointer())
            }
            Read(o) => {
                *self.get_cell(o)? = *self.get_input()?;
                Ok(self.advance_pointer())
            }
            Write(o) => {
                self.output.push(self.access(o)?);
                Ok(self.advance_pointer())
            }
            Jump(s) => Ok(self.jump_to(s)?),
            Jgtz(s) => {
                if self.tape[0] > 0 {
                    self.jump_to(s)?;
                    return Ok(RunState::Running);
                }
                Ok(self.advance_pointer())
            }
            Jzero(s) => {
                if self.tape[0] == 0 {
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
