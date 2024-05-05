use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() ->  io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Client connected");

        handle_client(&mut stream)?;
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hi from server";
    stream.write_all(response.as_bytes())?;
    println!("Sent response: {}", response);

    Ok(())
}
