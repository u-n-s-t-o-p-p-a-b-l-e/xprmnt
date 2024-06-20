use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientList = Arc<Mutex<Vec<TcpStream>>>;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Chat server listening on port 7878");

    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Failed to accept a client: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream, clients: ClientList) ->  io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client connected: {}", peer_addr);

    {
        let mut clients = clients.lock().unwrap();
        clients.push(stream.try_clone()?);
    }

    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let message = match line {
            Ok(msg) => msg,
            Err(_) => break,
        };

        println!("Received from {}: {}", peer_addr, message);
        broadcast_message(&message, &peer_addr, &clients)?;
    }

    {
        let mut clients = clients.lock().unwrap();
        clients.retain(|client| client.peer_addr().unwrap() != peer_addr);
    }

    println!("Client disconnected: {}", peer_addr);
    Ok(())
}
