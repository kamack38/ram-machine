use crate::parser::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct RamCode {
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<String, usize>,
}

impl RamCode {
    pub fn new() -> RamCode {
        RamCode {
            instructions: Vec::new(),
            jump_table: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}
