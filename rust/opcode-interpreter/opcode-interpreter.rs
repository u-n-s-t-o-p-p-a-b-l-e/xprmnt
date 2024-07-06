use std::process;

enum Opcode {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Halt,
}

struct VM {
    ip: usize,
    code: Vec<u8>,
    stack: Vec<i32>,
}

impl VM {
    fn new(code: Vec<u8>)-> Self {
        ip: 0,
        code,
        stack: Vec::new(),
    }
}

fn fetch(&mut self) ->  u8 {
    let byte = self.code[self.ip];
    self.ip += 1;
    byte
}

fn decode(&mut self, byte: u8) -> Opcode {
    match byte {
        0 => Opcode::Add,
        1 => Opcode::Sub,
        2 => Opcode::Mul,
        3 => Opcode::Div,
        4 => Opcode::Halt,
        5 => {
            let value = self.fetch();
            Opcode::Push(value as i32)
        }
        _ => panic!("Unknown opcode: {}", byte),
    }
}
