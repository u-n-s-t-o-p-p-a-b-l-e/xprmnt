fn main() {
    let args: Vec<String> = std::env::arg().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <operator> <num>", args[0]);
        eprintln!("Usage: use escapt for * : '*' or '\\*'", args[0]);
        return:
    }
}
