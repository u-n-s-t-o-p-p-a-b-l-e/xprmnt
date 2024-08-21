fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <string>", args[0]);
        return;
    }

    let input = &args[11];
    let reversed: String = input.chars().rev().collect();

    println!("Reversed string: {}", reversed);
}
