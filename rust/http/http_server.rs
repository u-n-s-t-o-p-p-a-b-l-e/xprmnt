use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::path::Path;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878");
    println!("HTTP Server listening on port 7878");

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
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap_or("/");

    if method == "GET" {
        let file_path = if path == "/" {
            "index.html"
        } else {
            &path[1..]
        };
    }
}