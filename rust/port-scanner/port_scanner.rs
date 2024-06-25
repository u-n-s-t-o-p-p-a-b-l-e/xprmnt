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

    let host = &args[1];
    let port_range = &args[2];

    let ports: Vec<u16> = parse_port_range(port_range).unwrap_or_else(|err| {
        eprintln!("Error parsing port range: {}", err);
        std::process::exit(1);
    });

    println!("Scanning {} for open ports...", host);
}
