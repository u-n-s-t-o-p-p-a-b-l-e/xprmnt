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
