use std::fs::File;
use std::io::{self, Write, Read};
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

fn handle_client(mut stream: TcpStream) ->  io::Result<()> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let filename = std::str::from_utf8(&buffer[..bytes_read]).expect("Invalid UTF-8");
    let filename = filename.trim();
    
    let mut file = File::create(filename)?;
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    println!("File received: {}", filename);
    Ok(())
}

