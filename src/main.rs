use std::{collections::HashMap, fmt::Error};
mod register;
mod cpu;
mod command;

use cpu::Cpu;

fn main() {

let mut cpu=Cpu::new();
    match cpu.load_code("mov a 5") {
        Ok(_) => println!("Code ok"),
        Err(e) => println!("Errr : {:?}", e),
    };
}
