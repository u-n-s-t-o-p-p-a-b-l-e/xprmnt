use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() ->  io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server");

    let request = "Hi from client";
    stream.write_all(request.as_bytes())?;
    println!("Sent request: {}", request);

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..]);
    println!("Received response: {}", response);

    Ok(())
}
