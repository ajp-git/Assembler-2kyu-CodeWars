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

        /*for line in txt.lines() {
            match line[0..3] {
                // one param
                "inc" => self.code.push(value),
                "dec" =>, 
                // two params
                "mov" => ,
                "jnz" =>,
                _ => ""
            }
        }*/
        Err("Not implemented".to_string())
    }

    fn get_register(&self, r:char) -> Result<i32, String>{
        match self.regs.get(&r) {
            Some(reg) => Ok(reg.get_value()),
            None => Err(format!("Unknown register {}", r)),
        }
    }
    fn set_register_value(&mut self,r:char, val:i32) -> Result<i32, String>{
        match self.regs.get_mut(&r) {
            Some(reg) => Ok(reg.set_value(val)),
            None => Err(format!("Unknown register {}", r)),
        }
    }
}