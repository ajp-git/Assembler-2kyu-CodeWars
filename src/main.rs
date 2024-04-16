use std::{collections::HashMap};

use regex::Regex;

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

#[derive(PartialEq,Debug, Clone)]
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
    Jne(String),        // jne lbl - jump to the label lbl if the values of the previous cmp command were not equal.
    Je(String),         // je lbl - jump to the label lbl if the values of the previous cmp command were equal.
    Jge(String),        // jge lbl - jump to the label lbl if x was greater or equal than y in the previous cmp command.
    Jg(String),         // jg lbl - jump to the label lbl if x was greater than y in the previous cmp command.
    Jle(String),        // jle lbl - jump to the label lbl if x was less or equal than y in the previous cmp command.
    Jl(String),         // jl lbl - jump to the label lbl if x was less than y in the previous cmp command.
    Call(String),       // call lbl - call to the subroutine identified by lbl. When a ret is found in a subroutine, the instruction pointer should return to the instruction next to this call command.
    Ret,                // ret - when a ret is found in a subroutine, the instruction pointer should return to the instruction that called the current function.
    Msg(String),   // msg 'Register: ', x - this instruction stores the output of the program. It may contain text strings (delimited by single quotes) and registers. The number of arguments isn't limited and will vary, depending on the program.
    End,                // end - this instruction indicates that the program ends correctly, so the stored output is returned (if the program terminates without this instruction it should return the default output: see below).
    Comment,            // ; comment - comments should not be taken in consideration during the execution of the program.
}

#[derive(Debug, PartialEq, Clone)]
enum Comparison {
    Equal,
    Less,
    Greater
}

#[derive(Clone)]
struct Cpu{
    regs:HashMap<char, Register>,
    code:Vec<Command>,
    compare:Option<Comparison>,
    labels:HashMap<String,usize>,
    sub_calls:Vec<usize>,
}

impl Cpu {
    fn new() -> Self {

        let registers:HashMap<char, Register>=HashMap::new();
        Cpu { regs: registers,
             code: Vec::new(),
             compare:None,
             labels:HashMap::new(),
            sub_calls: Vec::new() }
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

        let re_trim=Regex::new(r"\s+").unwrap();
        for (i, line) in txt.lines().enumerate() {
            let line=re_trim.replace_all(line, " ");
            println!("Processing line {}: {}", i, line); // Debug output
            if line.starts_with("msg") {
                self.code.push(Command::Msg(line.chars().skip(4).map(|c| c).collect::<String>()));
                continue;
            }
            if line.len()==0{continue;}
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
                ["cmp", x, y] => {
                    self.code.push(Command::Cmp(param(x), param(y)))
                },
                ["jmp", x] => {
                    self.code.push(Command::Jmp(x.to_string()));
                },
                [label] if label.ends_with(':')=> {
                    let label_name = label.trim_end_matches(':');
                    self.labels.insert(label_name.to_string(), i);
                    self.code.push(Command::Label(label_name.to_string()));
                },
                ["jne", x] => {
                    self.code.push(Command::Jne(x.to_string()));
                },
                ["je", x] => {
                    self.code.push(Command::Je(x.to_string()));
                },
                ["jge", x] => {
                    self.code.push(Command::Jge(x.to_string()));
                },
                ["jg", x] => {
                    self.code.push(Command::Jg(x.to_string()));
                },
                ["jle", x] => {
                    self.code.push(Command::Jle(x.to_string()));
                },
                ["jl", x] => {
                    self.code.push(Command::Jl(x.to_string()));
                },
                ["call", x] => {
                    self.code.push(Command::Call(x.to_string()));
                },
                ["ret"] => {
                    self.code.push(Command::Ret);
                },
                ["end"] => {
                    self.code.push(Command::End);
                },
                [";"] => {
                    self.code.push(Command::Comment);
                },
                _ => panic!("Unknown instruction {}", line),
            }
        }
        Ok(())
    }

    fn print_status(self: &Self, address: &usize, code:&Command){
        println!("Address : {}\t{:?}", address,code);
        for (c,reg) in &   self.regs {
            println!("{} : {}", c, reg.val);
        }
    }
    fn run (&mut self) -> Result<String,String>{
        let mut address=0;
        let mut out:String=String::new();

        while address < self.code.len(){
            let code= self.code[address].clone();
            //self.print_status(&address, &code);
            match code {

                Command::Dec(a) => {
                    self.parse_register(&a)?.dec();
                },
                Command::Inc(a) => {
                    self.parse_register(&a)?.inc();
                },
                Command::Move(a, b) => {
                    self.set_register_value(a,self.get_param_value(&b)?);
                },
                Command::Jnz(a, b) => {
                    let condition=self.get_param_value(&a)? as usize;
                    let jump = self.get_param_value(&b)?;
                    if condition !=0 {
                        if jump < 0 && address< jump.abs() as usize {
                            return Err(format!("Bad jump from {} -> {}", address, jump));
                        }
                        address = address.wrapping_add(jump as usize);
                        continue;
                    }
                },
                Command::Add(r, p) => {
                    let val = self.get_param_value(&p)?;
                    self.parse_register(&r)?.add(val);
                },
                Command::Sub(r, p) => {
                    let val = self.get_param_value(&p)?;
                    self.parse_register(&r)?.sub(val);
                },
                Command::Mul(r, p) => {
                    let val = self.get_param_value(&p)?;
                    self.parse_register(&r)?.mul(val);
                },
                Command::Div(r, p) => {
                    let val = self.get_param_value(&p)?;
                    self.parse_register(&r)?.div(val);
                },
                Command::Cmp(p1, p2) => {
                    let val_1 = self.get_param_value(&p1)?;
                    let val_2 = self.get_param_value(&p2)?;
                    if val_1==val_2 {self.compare=Some(Comparison::Equal);}
                    else if val_1<val_2 { self.compare=Some(Comparison::Less);}
                    else { self.compare=Some(Comparison::Greater);}
                },
                Command::Label(x) => {},
                Command::Jmp(x) => {
                    address=self.get_label_address(&x); continue;
                },
                Command::Jne(x) => {
                    if self.compare!=None && self.compare!=Some(Comparison::Equal) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Je(x) => {
                    if self.compare==Some(Comparison::Equal) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Jge(x) => {
                    if self.compare==Some(Comparison::Equal) || self.compare==Some(Comparison::Greater) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Jg(x) => {
                    if self.compare==Some(Comparison::Greater) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Jle(x) => {
                    if self.compare==Some(Comparison::Equal) || self.compare==Some(Comparison::Less) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Jl(x) => {
                    if self.compare==Some(Comparison::Less) {
                        address=self.get_label_address(&x); continue;
                    }
                },
                Command::Call(x) => {
                    self.sub_calls.push(address);
                    address=self.get_label_address(&x.to_string());
                    continue;
                },
                Command::Ret => {
                    address=self.sub_calls.pop().unwrap();
                },
                Command::Msg(x) => {
                    print!("Msg : ");
                    let mut s = String::new();
                    let mut in_text=false;
                    for c in x.chars() {
                        match c {
                            '\'' => {
                                in_text=!in_text;
                            },
                            _ if in_text => s.push(c),
                            'a'..='z' if in_text==false => {
                                let o = format!("{}", self.get_register_value(&c).unwrap_or(0));
                                s.push_str(o.as_str());
                            },
                            _ => {
                                println!("in message, '{}' ignored",c);
                            },
                            
                        }
                    }
                    out.push_str(s.as_str());
                    println!("Current out :{}", out);
                    
//                    x.iter().for_each(|f|print!("{}", f));
                },
                Command::End => {return Ok(out)},
                _ => todo!(),
            }
            address+=1;
        }
        Ok(out)
    }

    fn get_register_value(&mut self, r:&char) -> Result<i64, String>{
        
        if let Ok(reg)=self.parse_register(r) {
            return Ok(reg.get_value());
        }
        Err(format!("Get register {} value error", r))
    }

    fn get_param_value(&self, p:&Param) -> Result<i64, String>{
        match p {
            Param::Register(r) => { Ok(self.regs.get(&r).unwrap().val) },
            Param::Val(v) => Ok(*v),
        }
    }
    
    fn set_register_value(&mut self,r:char, val:i64) {
        let mut reg=self.regs.entry(r).or_insert(Register{val:0});
        reg.set_value(val);
        
    }

    fn parse_register(&mut self, input: &char) -> Result<&mut Register, String> {
        if input.is_alphabetic()  {
//            let reg_name= input.chars().next().unwrap();
            let reg= self.regs.entry(*input).or_insert_with(|| Register{val:0});
            return Ok(reg);
        } else {
            return Err(format!("Unknown register {}", input));
        }
    }

    fn get_label_address (&self, label:&String) -> usize {
        *self.labels.get(label).unwrap()
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
        let reg=cpu.parse_register(&'a').unwrap();
        assert_eq!(cpu.code[0], Command::Inc('a'));
    }

    
    #[test]
    fn test_cpu_load_code_from_vec() {
        let mut cpu = init_cpu();
        let code = ["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        cpu.load_code_from_vec(&code);
        cpu.run();
        assert_eq!(cpu.get_register_value(&'a'), Ok(1));
    }
    #[test]
    fn test_cpu_operations() {
        let mut cpu = init_cpu();
        let code = ["mov a 5", "sub a 2", "add a 3", "mul a 2", "mov b a", "div a b"];
        cpu.load_code_from_vec(&code);
        cpu.run();
        assert_eq!(cpu.get_register_value(&'a'), Ok(1));
    }

    #[test]
    fn test_labels() {
        let mut cpu = init_cpu();
        let code = ["coucou:", "mov a 3", "first:", "second:"];
        let _ = cpu.load_code_from_vec(&code);
        let _ = cpu.run();
        assert_eq!(*cpu.labels.get(&"second".to_string()).unwrap(), 3 as usize);
    }

    #[test]
    fn test_jump() {
        let mut cpu = init_cpu();
        let code = ["mov a 3", "coucou:", "dec a", "cmp a 0", "jne coucou", "end"];
        let _ = cpu.load_code_from_vec(&code);
        let _ = cpu.run();
        assert_eq!(cpu.regs.get(&'a').unwrap().val, 0);
    }
    

    #[test]
    fn test_sub() {
        let mut cpu = init_cpu();
        let code = ["mov a 3", "coucou:","call sub_dec", "cmp a 0", "jne coucou", "end", "sub_dec:", "dec a", "ret"];
        let _ = cpu.load_code_from_vec(&code);
        let _ = cpu.run();
        assert_eq!(cpu.regs.get(&'a').unwrap().val, 0);
    }

    #[test]
    fn test_msg() {
        let mut cpu = init_cpu();
        let code = ["mov a 3", "mov b 2", "mov c 6", "msg   'mul(', a, ', ', b, ') = ', c        "];
        let _ = cpu.load_code_from_vec(&code);
        let out = cpu.run().unwrap();
        assert_eq!(out, "mul(3, 2) = 6");
    }
    
    #[test]
    fn test_cw1() {
        let mut cpu = init_cpu();
        let code = "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n";
        let _ = cpu.load_code(&code);
        let out = cpu.run().unwrap();
        assert_eq!(out, "mul(3, 2) = 6");
    }
    
}