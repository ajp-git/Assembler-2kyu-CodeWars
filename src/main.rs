use std::{collections::HashMap, fmt::Error};

#[derive(Clone, Copy, Debug)]
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

enum Param {
    Val(i32),
    Register(char),  
  } 
  
enum Command {
    Move(Param,Register),
    Inc(Register),
    Dec(Register),
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

    fn load_code(&mut self, txt: &str) -> Result<(), String> {

        for line in txt.lines() {
            let parts:Vec<&str>=line.split_whitespace().collect();
            match parts.as_slice() {
                ["inc", x] => {
                    let reg=self.parse_register(x)?;
                    self.code.push(Command::Inc(reg));    
                },
                
                ["dec", x] => {
                    let reg=self.parse_register(x)?;
                    self.code.push(Command::Inc(reg));    
                },
                
                // two params
                ["mov", x, y] => {
                    let param = self.parse_param(&x)?;
                    let reg = self.parse_register(&y)?;
                    self.code.push(Command::Move(param, reg));

                },
                ["jnz", x, y] =>{
                    let param =self.parse_param(&x)?;
                    let jump=self.parse_param(&y)?;
                    self.code.push(Command::Jnz(param, jump));
                },
                _ => panic!("Unknown instruction {}", line),
            }
        }
        Err("Not implemented".to_string())
    }

    fn get_register_value(&mut self, r:&str) -> Result<i32, String>{
        
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
    
    fn set_register_value(&mut self,r:char, val:i32) -> Result<i32, String>{
        match self.regs.get_mut(&r) {
            Some(reg) => Ok(reg.set_value(val)),
            None => Err(format!("Unknown register {}", r)),
        }
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

    fn parse_register(&mut self, input: &str) -> Result<Register, String> {
        if input.len()==1 && input.chars().next().unwrap().is_alphabetic()  {
            let reg_name= input.chars().next().unwrap();
            let reg= self.regs.entry(reg_name).or_insert_with(|| Register{val:0});
            return Ok(*reg);
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
    fn test_cpu_mov() {
        let mut register = Register { val: 42 };
        register.dec();
        assert_eq!(register.val, 41);
    }
}