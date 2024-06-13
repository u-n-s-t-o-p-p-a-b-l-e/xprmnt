use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<Strng, TcpStream>>>;

fn main() ->  io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Server running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::Spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(e) => {
                eprintln!("Eror: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream, clients: Clients) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();

    if reader.read_line(&mut name).is_ok() {
        name = name.trim().to_string();
        println!("{} has joined", name);

        clients.lock().unwrap().insert(name.clone(), stream.try_clone().unwrap());

        let clients = Arc::clone(&clients);
        let stream_clone = stream.try_clone().unwrap();

        thread::spawn(move || {
            broadcast_messages(name.clone(), clients, reader);
        });

        let mut stream = stream_clone;
        loop {
            let mut message = String::new();
            if reader.read_line(&mut message).is_err() {
                break;
            }
            let message = message.trim().to_string();
            if message.is_empty() {
                continue;
            }

            let message_to_send = format1!("{}: {}\n", name, message);
            let clients = clients.lock().unwrap();
            for (client_name, client_stream) in clients.iter() {
                if client_name != &name {
                    let _ = client_stream.write_all(message_to_send.as_bytes());
                }
            }
        }

        clients.lock().unwrap().remove(&name);
        println!("{} has left", name);
    }
}

fn broadcast_messages(name: String, clients: Clients, reader: BufReader<TcpStream>) {
    let clients = Arc::clone(&clients);
    let mut reader = reader;

    loop {
        let mut message = String::new();
        if reader.read_line(&mut message).is_err() {
            break;
        }
    }
}
