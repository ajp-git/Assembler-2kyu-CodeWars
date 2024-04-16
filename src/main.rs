use std::{collections::HashMap};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Register {
    pub val:i64,
}

impl Register {
    fn set_value(&mut self, new_val:i64 ) -> i64 {
        self.val=new_val;
        self.val
    }

    fn get_value(&self ) -> i64 {
        self.val
    }

    fn inc(&mut self) {
        self.val+=1;
    }

    fn dec(&mut self) {
        self.val-=1;
    }

    fn add(&mut self, val:i64) {
        self.val+=val;
    }
    fn sub(&mut self, val:i64) {
        self.val-=val;
    }
    fn mul(&mut self, val:i64) {
        self.val*=val;
    }
    fn div(&mut self, val:i64) {
        self.val/=val;
    }
}

#[derive(PartialEq,Debug,Clone, Copy)]
enum Param {
    Val(i64),
    Register(char),  
  } 

#[derive(PartialEq,Debug)]
  enum Command {
    Move(char, Param),
    Inc(char),
    Dec(char),
    Jnz(Param,Param),
    Add(char,Param),    // add x, y - add the content of the register x with y (either an integer or the value of a register) and stores the result in x (i.e. register[x] += y).
    Sub(char, Param),   // sub x, y - subtract y (either an integer or the value of a register) from the register x and stores the result in x (i.e. register[x] -= y).
    Mul(char, Param),   // mul x, y - same with multiply (i.e. register[x] *= y).
    Div(char, Param),   // div x, y - same with integer division (i.e. register[x] /= y).
    Label(String),      // label: - define a label position (label = identifier + ":", an identifier being a string that does not match any other command). Jump commands and call are aimed to these labels positions in the program.
    Jmp(String),        // jmp lbl - jumps to the label lbl.
    Cmp(Param,Param),   // cmp x, y - compares x (either an integer or the value of a register) and y (either an integer or the value of a register). The result is used in the conditional jumps (jne, je, jge, jg, jle and jl)
}

struct Cpu{
    regs:HashMap<char, Register>,
    code:Vec<Command>,
    compare:bool,
}

impl Cpu {
    fn new() -> Self {

        let registers:HashMap<char, Register>=HashMap::new();
        Cpu { regs: registers, code: Vec::new(), compare:false }
    }

    fn load_code_from_vec(&mut self, code:&[&str]) -> Result<(), String>{

        self.load_code(code.join("\n").as_str())

    }

    fn load_code(&mut self, txt: &str) -> Result<(), String> {
        println!("Input to load_code:\n{}", txt); // Debug output

        let reg = |x:&str| x.chars().next().unwrap();
        
        let param = |y:&str|{
            if let Ok(val)=y.parse::<i64>() {
                return Param::Val(val);
            } else if y.len()==1 && y.chars().next().unwrap().is_alphabetic() {
                let reg=y.chars().next().unwrap();
                return Param::Register(reg);
            } else {
                panic!("Bad param {}", y);
            }
        };
    
        for (i, line) in txt.lines().enumerate() {
            println!("Processing line {}: {}", i, line); // Debug output
            let parts:Vec<&str>=line.split_whitespace().collect();
            match parts.as_slice() {
                ["inc", x] => {
                    self.code.push(Command::Inc(reg(x)));    
                },
                ["dec", x] => {
                    self.code.push(Command::Dec(reg(x)));    
                },
                ["mov", x, y] => {
                    self.code.push(Command::Move(reg(x), param(y)));
                },
                ["jnz", x, y] =>{
                    self.code.push(Command::Jnz(param(x), param(y)));
                },
                ["add", x, y] => {
                    self.code.push(Command::Add(reg(x), param(y)))
                },
                ["sub", x, y] => {
                    self.code.push(Command::Sub(reg(x), param(y)))
                },
                ["mul", x, y] => {
                    self.code.push(Command::Mul(reg(x), param(y)))
                },
                ["div", x, y] => {
                    self.code.push(Command::Div(reg(x), param(y)))
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

                Command::Dec(a) => {
                    self.parse_register(*a)?.dec();
                },
                Command::Inc(a) => {
                    self.parse_register(*a)?.inc();
                },
                Command::Move(a, b) => {
                    self.set_register_value(*a,self.get_param_value(*b)?);
                },
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
                Command::Add(r, p) => {
                    let val = self.get_param_value(*p)?;
                    self.parse_register(*r)?.add(val);
                },
                Command::Sub(r, p) => {
                    let val = self.get_param_value(*p)?;
                    self.parse_register(*r)?.sub(val);
                },
                Command::Mul(r, p) => {
                    let val = self.get_param_value(*p)?;
                    self.parse_register(*r)?.mul(val);
                },
                Command::Div(r, p) => {
                    let val = self.get_param_value(*p)?;
                    self.parse_register(*r)?.div(val);
                },
                _ => todo!(),
            }
            address+=1;
        }
        Ok(address)
    }

    fn get_register_value(&mut self, r:char) -> Result<i64, String>{
        
        if let Ok(reg)=self.parse_register(r) {
            return Ok(reg.get_value());
        }
        Err(format!("Get register {} value error", r))
    }

    fn get_param_value(&self, p:Param) -> Result<i64, String>{
        match p {
            Param::Register(r) => { Ok(self.regs.get(&r).unwrap().val) },
            Param::Val(v) => Ok(v),
        }
    }
    
    fn set_register_value(&mut self,r:char, val:i64) {
        let mut reg=self.regs.entry(r).or_insert(Register{val:0});
        reg.set_value(val);
        
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

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();

    let mut cpu = Cpu::new();
    cpu.load_code_from_vec(&program);
    cpu.run();

    cpu.regs.into_iter().for_each(|(c,reg)|{registers.insert(c.to_string(), reg.val);});

    registers
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
    fn test_register_mul() {
        let mut register = Register { val: 42 };
        register.mul(2);
        assert_eq!(register.val, 84);
    }
    #[test]
    fn test_register_div() {
        let mut register = Register { val: 42 };
        register.div(2);
        assert_eq!(register.val, 21);
    }
    #[test]
    fn test_register_add() {
        let mut register = Register { val: 42 };
        register.add(2);
        assert_eq!(register.val, 44);
    }
    #[test]
    fn test_register_sub() {
        let mut register = Register { val: 42 };
        register.sub(2);
        assert_eq!(register.val, 40);
    }
    
    #[test]
    fn test_cpu_load_code_inc() {
        let mut cpu = init_cpu();
        cpu.load_code("inc a");
        let reg=cpu.parse_register('a').unwrap();
        assert_eq!(cpu.code[0], Command::Inc('a'));
    }

    
    #[test]
    fn test_cpu_load_code_from_vec() {
        let mut cpu = init_cpu();
        let code = ["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        cpu.load_code_from_vec(&code);
        cpu.run();
        assert_eq!(cpu.get_register_value('a'), Ok(1));
    }
    #[test]
    fn test_cpu_operations() {
        let mut cpu = init_cpu();
        let code = ["mov a 5", "sub a 2", "add a 3", "mul a 2", "mov b a", "div a b"];
        cpu.load_code_from_vec(&code);
        cpu.run();
        assert_eq!(cpu.get_register_value('a'), Ok(1));
    }

    
}