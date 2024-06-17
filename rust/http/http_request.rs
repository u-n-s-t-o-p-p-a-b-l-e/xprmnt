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

    let url = &args[1];
    match fetch_url(url) {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn fetch_url(url: &str) ->  Result<String, Box<dyn std::error::Error>> {
    let (host, path) = parse_url(url)?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host)
};
