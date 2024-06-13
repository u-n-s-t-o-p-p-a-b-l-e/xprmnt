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
