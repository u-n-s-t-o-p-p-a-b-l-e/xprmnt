use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
}
