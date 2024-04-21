use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Hi, {}!", args[1]);
    } else {
        println!("Please enter your name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        let name = name.trim();

        println!("Hi, {}", name);
    }
}
