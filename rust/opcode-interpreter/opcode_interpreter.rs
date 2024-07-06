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
    fn new(code: Vec<u8>) -> Self {
        Self {
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

    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Push(value) => self.stack.push(value),
            Opcode::Add => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a + b);
            }
            Opcode::Sub => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a - b);
            }
            Opcode::Mul => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a * b);
            }
            Opcode::Div => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a / b);
            }
            Opcode::Halt => {
                println!("Result: {}", self.stack.pop().expect("Stack underflow"));
                process::exit(0);
            }
        }
    }

    fn run(&mut self) {
        loop {
            let byte = self.fetch();
            let opcode = self.decode(byte);
            self.execute(opcode);
        }
    }
}

fn main() {
    let code = vec![
        5, 3,
        5, 2,
        0,
        5, 4,
        2,
        5, 5,
        5, 2,
        3,
        1,
        4,
    ];

    let mut vm = VM::new(code);
    vm.run();
}













