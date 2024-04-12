use std::collections::HashMap;
use crate::register::Register;
use crate::command::Command;

pub struct Cpu{
    regs:HashMap<char, Register>,
    code:Vec<Command>,
}

impl Cpu {
    pub fn new() -> Self {

        let registers:HashMap<char, Register>=HashMap::new();
        Cpu { regs: registers, code: Vec::new() }
    }

    pub fn load_code(&mut self, txt: &str) -> Result<(), String> {

        for line in txt.lines() {
            
        }
        Err("Not implemented".to_string())
    }
}