use std::io::{self, Read, Write};
use std::process::exit;

fn main() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();

    println!("Welcome to Rust Editor!");
    println!("Press 'q' to quit.");
}
