use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server");
}
