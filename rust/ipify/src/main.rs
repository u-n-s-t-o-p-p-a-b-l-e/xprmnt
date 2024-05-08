use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    // Bind to the loopback address 127.0.0.1
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Client connected from: {}", stream.peer_addr()?);

        handle_client(&mut stream)?;
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // End of stream, client disconnected
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);

        let response = "Hi from server\n";
        stream.write_all(response.as_bytes())?;
        println!("Sent response: {}", response);
    }
    Ok(())
}

