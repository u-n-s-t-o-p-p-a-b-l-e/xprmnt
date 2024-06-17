use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
}
