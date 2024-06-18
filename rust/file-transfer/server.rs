use std::fs::File;
use std::io::{self, Write, Read, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Failed to accept a client: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut filename = String::new();
    
    // Read the filename
    reader.read_line(&mut filename)?;
    let filename = filename.trim();  // Remove any newline characters

    let mut file = File::create(&filename)?;

    // Read the file content
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    file.write_all(&buffer)?;

    println!("File received: {}", filename);
    Ok(())
}

