use crate::parser::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct RAMCode {
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<String, u32>,
}

impl RAMCode {
    pub fn new() -> RAMCode {
        RAMCode {
            instructions: Vec::new(),
            jump_table: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}
