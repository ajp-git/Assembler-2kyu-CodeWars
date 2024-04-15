use std::{collections::HashMap, fmt::Error};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Register {
    pub val:i32,
}

impl Register {
    fn set_value(&mut self, new_val:i32 ) -> i32 {
        self.val=new_val;
        self.val
    }

    fn get_value(&self ) -> i32 {
        self.val
    }

    fn inc(&mut self) -> i32 {
        self.val+=1;
        self.val
    }

    fn dec(&mut self) -> i32 {
        self.val-=1;
        self.val
    }
}

#[derive(PartialEq,Debug,Clone, Copy)]
enum Param {
    Val(i32),
    Register(char),  
  } 

#[derive(PartialEq,Debug)]
  enum Command {
    Move(char, Param),
    Inc(char),
    Dec(char),
    Jnz(Param,Param),
}

struct Cpu{
    regs:HashMap<char, Register>,
    code:Vec<Command>,

}

impl Cpu {
    fn new() -> Self {

        let registers:HashMap<char, Register>=HashMap::new();
        Cpu { regs: registers, code: Vec::new() }
    }

    fn load_code_from_vec(&mut self, code:&[&str]) -> Result<(), String>{

        self.load_code(code.join("\n").as_str())

    }

    fn load_code(&mut self, txt: &str) -> Result<(), String> {
        println!("Input to load_code:\n{}", txt); // Debug output

        for (i, line) in txt.lines().enumerate() {
            println!("Processing line {}: {}", i, line); // Debug output
            let parts:Vec<&str>=line.split_whitespace().collect();
            match parts.as_slice() {
                ["inc", x] => {
                    let reg=x.chars().next().unwrap();
                    self.code.push(Command::Inc(reg));    
                },
                
                ["dec", x] => {
                    let reg=x.chars().next().unwrap();
                    self.code.push(Command::Dec(reg));    
                },
                
                // two params
                ["mov", x, y] => {
                    let reg=x.chars().next().unwrap();
                    let param = self.parse_param(&y)?;
                    self.code.push(Command::Move(reg, param));

                },
                ["jnz", x, y] =>{
                    let param =self.parse_param(&x)?;
                    let jump=self.parse_param(&y)?;
                    self.code.push(Command::Jnz(param, jump));
                },
                _ => panic!("Unknown instruction {}", line),
            }
        }
        Ok(())
    }

    fn run (&mut self) -> Result<usize,String>{
        let mut address=0;

        while address < self.code.len(){
            match &self.code[address] {

                Command::Dec(a) => {self.parse_register(*a)?.dec();},
                Command::Inc(a) => {self.parse_register(*a)?.inc();},
                Command::Move(a, b) => {self.set_register_value(*a,self.get_param_value(*b)?);},
                Command::Jnz(a, b) => {
                    let condition=self.get_param_value(*a)? as usize;
                    let jump = self.get_param_value(*b)?;
                    if condition !=0 {
                        if jump < 0 && address< jump.abs() as usize {
                            return Err(format!("Bad jump from {} -> {}", address, jump));
                        }
                        address = address.wrapping_add(jump as usize);
                        continue;
                    }
                },
            }
            address+=1;
        }
        Ok(address)
    }

    fn get_register_value(&mut self, r:char) -> Result<i32, String>{
        
        if let Ok(reg)=self.parse_register(r) {
            return Ok(reg.get_value());
        }
        Err(format!("Get register {} value error", r))
    }

    fn get_param_value(&self, p:Param) -> Result<i32, String>{
        match p {
            Param::Register(r) => { Ok(self.regs.get(&r).unwrap().val) },
            Param::Val(v) => Ok(v),
        }
    }
    
    fn set_register_value(&mut self,r:char, val:i32) {
        let mut reg=self.regs.entry(r).or_insert(Register{val:0});
        reg.set_value(val);
        
    }

    fn parse_param(&self, input:&str) -> Result<Param,String> {
        if let Ok(val)=input.parse::<i32>() {
            return Ok(Param::Val(val));
        } else if input.len()==1 && input.chars().next().unwrap().is_alphabetic() {
            let reg=input.chars().next().unwrap();
            return Ok(Param::Register(reg));
        } else {
            return Err(format!("Bad param {}", input));
        }
    }

    fn parse_register(&mut self, input: char) -> Result<&mut Register, String> {
        if input.is_alphabetic()  {
//            let reg_name= input.chars().next().unwrap();
            let reg= self.regs.entry(input).or_insert_with(|| Register{val:0});
            return Ok(reg);
        } else {
            return Err(format!("Unknown register {}", input));
        }
    }

}
fn main() {

let mut cpu=Cpu::new();
    match cpu.load_code("mov a 5") {
        Ok(_) => println!("Code ok"),
        Err(e) => println!("Errr : {:?}", e),
    };
}

#[cfg(test)]
mod tests {

    use crate::Param;

    use super::Register;
    use super::Cpu;
    use super::Command;

    fn init_cpu() -> Cpu {
        Cpu::new()
    }

    #[test]
    fn test_register_set_value() {
        let mut register = Register { val: 0 };
        register.set_value(42);
        assert_eq!(register.val, 42);
    }

    #[test]
    fn test_register_get_value() {
        let register = Register { val: 42 };
        assert_eq!(register.get_value(), 42);
    }

    #[test]
    fn test_register_inc() {
        let mut register = Register { val: 0 };
        register.inc();
        assert_eq!(register.val, 1);
    }

    #[test]
    fn test_register_dec() {
        let mut register = Register { val: 42 };
        register.dec();
        assert_eq!(register.val, 41);
    }
    
    #[test]
    fn test_cpu_load_code_inc() {
        let mut cpu = init_cpu();
        cpu.load_code("inc a");
        let reg=cpu.parse_register('a').unwrap();
        assert_eq!(cpu.code[0], Command::Inc('a'));
    }

    #[test]
    fn test_cpu_load_code_mov_regs() {
        let mut cpu = init_cpu();
        cpu.load_code("mov a b");
        let reg_2=cpu.parse_param("b").unwrap();
        assert_eq!(cpu.code[0], Command::Move('a', reg_2));
    }


    #[test]
    fn test_cpu_load_code_mov_val() {
        let mut cpu = init_cpu();
        cpu.load_code("mov b -1");
        let val=cpu.parse_param("-1").unwrap();
        assert_eq!(cpu.code[0], Command::Move('b', val));
    }
    
    #[test]
    fn test_cpu_load_code_from_vec() {
        let mut cpu = init_cpu();
        let code = ["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        cpu.load_code_from_vec(&code);
        cpu.run();
        assert_eq!(cpu.get_register_value('a'), Ok(1));
    }

    
}