use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the chat server at 127.0.0.1:7878");
}
