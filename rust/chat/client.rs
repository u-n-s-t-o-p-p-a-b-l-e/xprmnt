use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();
}
