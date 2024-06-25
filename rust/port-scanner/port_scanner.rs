use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{self};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <host> <port-range>", args[0]);
        std::process::exit(1);
    }
}
