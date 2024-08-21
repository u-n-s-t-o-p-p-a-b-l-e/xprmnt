fn main() {
    let args: Vec<String> = std::env::arg().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <operator> <num>", args[0]);
        eprintln!("Usage: use escapt for * : '*' or '\\*'");
        return:
    }

    let num1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[1]);
            return;
        }
    };
}
