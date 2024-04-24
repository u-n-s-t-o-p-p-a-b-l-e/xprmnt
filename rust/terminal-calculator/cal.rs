use std::io::{self, Write};

fn main() {
    loop {
        println!("Calculator");
        println!("1. Add");
        println!("2. Substract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        print!("\r\nEnter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => perform_operation('+'),
            "2" => perform_operation('-'),
            "3" => perform_operation('*'),
            "4" => perform_operation('/'),
            "5" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn perform_operation(op: char) {
    let mut input = String::new();
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Error: Division by zero");
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("\r\nResult: {}\r\n", result);
}
