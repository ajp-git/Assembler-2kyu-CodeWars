use crate::register::Register;

enum Param {
    Val(i32),
    Register(char),  
  } 
  
pub enum Command {
    Move(Param,Register),
}
