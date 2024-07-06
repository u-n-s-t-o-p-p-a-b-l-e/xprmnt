use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
   let mut buffer = [0; 1024] ;
   stream.read(&mut buffer).unwrap();

   let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHi World!";
   stream.write(response.as_bytes()).unwrap();
   stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
