
# Fun exercice to read assembly code and make it run.

## Structs and enums created

### Register

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct  Register {
    pub  val:i64,
    }
### Parameter
    #[derive(PartialEq,Debug,Clone, Copy)]
    enum  Param {
    Val(i64),
    Register(char),
    }
### Supported Command
    #[derive(PartialEq,Debug, Clone)]
    enum  Command {
    Move(char, Param),
    Inc(char),
    Dec(char),
    Jnz(Param,Param),
    Add(char,Param), // add x, y - add the content of the register x with y (either an integer or the value of a register) and stores the result in x (i.e. register[x] += y).
    Sub(char, Param), // sub x, y - subtract y (either an integer or the value of a register) from the register x and stores the result in x (i.e. register[x] -= y).
    Mul(char, Param), // mul x, y - same with multiply (i.e. register[x] *= y).
    Div(char, Param), // div x, y - same with integer division (i.e. register[x] /= y).
    Label(String), // label: - define a label position (label = identifier + ":", an identifier being a string that does not match any other command). Jump commands and call are aimed to these labels positions in the program.
    Jmp(String), // jmp lbl - jumps to the label lbl.
    Cmp(Param,Param), // cmp x, y - compares x (either an integer or the value of a register) and y (either an integer or the value of a register). The result is used in the conditional jumps (jne, je, jge, jg, jle and jl)
    Jne(String), // jne lbl - jump to the label lbl if the values of the previous cmp command were not equal.
    Je(String), // je lbl - jump to the label lbl if the values of the previous cmp command were equal.
    Jge(String), // jge lbl - jump to the label lbl if x was greater or equal than y in the previous cmp command.
    Jg(String), // jg lbl - jump to the label lbl if x was greater than y in the previous cmp command.
    Jle(String), // jle lbl - jump to the label lbl if x was less or equal than y in the previous cmp command.
    Jl(String), // jl lbl - jump to the label lbl if x was less than y in the previous cmp command.
    Call(String), // call lbl - call to the subroutine identified by lbl. When a ret is found in a subroutine, the instruction pointer should return to the instruction next to this call command.
    Ret, // ret - when a ret is found in a subroutine, the instruction pointer should return to the instruction that called the current function.
    Msg(String), // msg 'Register: ', x - this instruction stores the output of the program. It may contain text strings (delimited by single quotes) and registers. The number of arguments isn't limited and will vary, depending on the program.
    End, // end - this instruction indicates that the program ends correctly, so the stored output is returned (if the program terminates without this instruction it should return the default output: see below).
    Comment, // ; comment - comments should not be taken in consideration during the execution of the program.
    }
### Comparison memory storage

    #[derive(Debug, PartialEq, Clone)]
    enum  Comparison {
    Equal,
    Less,
    Greater
    }

### Cpu 
    #[derive(Clone)]
    struct  Cpu{
    regs:HashMap<char, Register>,
    code:Vec<Command>,
    compare:Option<Comparison>,
    labels:HashMap<String,usize>,
    sub_calls:Vec<usize>,
    }

## Assembly code example

    ; Mod function
    mod_func:
        mov   c, a        ; temp1
        div   c, b
        mul   c, b
        mov   d, a        ; temp2
        sub   d, c
        ret
    
    mov   a, 81         ; value1
    mov   b, 153        ; value2
    call  init
    call  proc_gcd
    call  print
    end
    
    proc_gcd:
        cmp   c, d
        jne   loop
        ret
    
    loop:
        cmp   c, d
        jg    a_bigger
        jmp   b_bigger
    
    a_bigger:
        sub   c, d
        jmp   proc_gcd
    
    b_bigger:
        sub   d, c
        jmp   proc_gcd
    
    init:
        cmp   a, 0
        jl    a_abs
        cmp   b, 0
        jl    b_abs
        mov   c, a            ; temp1
        mov   d, b            ; temp2
        ret
    
    a_abs:
        mul   a, -1
        jmp   init
    
    b_abs:
        mul   b, -1
        jmp   init
    
    print:
        msg   'gcd(', a, ', ', b, ') = ', c
        ret
    
    call  func1
    call  print
    end


