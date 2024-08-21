fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <operator> <num>", args[0]);
        eprintln!("Usage: use escapt for * : '*' or '\\*'");
        return;
    }

    let num1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[1]);
            return;
        }
    };

    let num2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[3]);
            return;
        }
    };

    let operator = &args[2];
    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            eprintln!("Error: Unsupported operator '{}'", operator);
            return;
        }
    };

    println!("Result: {}", result);
}
