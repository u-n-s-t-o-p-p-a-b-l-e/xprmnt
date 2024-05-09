use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Client connected");

        thread::spawn(move || {
            if let Err(err) = handle_client(&mut stream) {
                eprintln!("Error handling client: {}", err);
            }
        });
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Client disconnected.");
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);

        stream.write_all(&buffer[..bytes_read]);
    }
}
