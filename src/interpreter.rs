use crate::parser::{
    instruction::Instruction,
    operand::{CellOperand, CellValue, ExpandError, Operand},
    CodeParseError, RamCode,
};
use tabled::{settings::Style, Table};
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
    #[error("Multiplication by `{0}` of `{1}` failed.")]
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

    pub fn from_str(str: &str, input: Vec<CellValue>) -> Result<Self, CodeParseError> {
        Ok(RamMachine::new(str.parse()?, input))
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

    pub fn run_line(&mut self) -> Result<RunState, RamMachineError> {
        let instruction = self.code.instructions[self.pointer].clone();
        self.execute(&instruction)
    }

    pub fn get_current_instruction(&self) -> &Instruction {
        &self
            .code
            .instructions
            .get(self.pointer)
            .unwrap_or(&Instruction::Halt)
    }

    pub fn print_state(&self) {
        let tab: Vec<String> = self
            .tape
            .iter()
            .map(|x| match x {
                Some(v) => v.to_string(),
                None => "?".to_string(),
            })
            .collect();
        let table = Table::builder(tab)
            .index()
            .transpose()
            .column(0)
            .build()
            .with(Style::rounded())
            .to_string();

        let next_instruction = self.get_current_instruction();

        println!("{table}");
        println!(
            "Input:{}",
            self.input
                .iter()
                .fold("".to_string(), |s, v| format!("{s} {v}"))
        );
        println!(
            "Output:{}",
            self.output
                .iter()
                .fold("".to_string(), |s, v| format!("{s} {v}"))
        );
        println!("Next instruction: {next_instruction}");
    }

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
                    return Ok(self.jump_to(s)?);
                }
                Ok(self.advance_pointer())
            }
            Jzero(s) => {
                if *self.buffer()? == 0 {
                    return Ok(self.jump_to(s)?);
                } else {
                    Ok(self.advance_pointer())
                }
            }
            Halt => Ok(RunState::Halted),
        }
    }
}
