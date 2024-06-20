use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the chat server at 127.0.0.1:7878");

    let read_stream = stream.try_clone()?;

    thread::spawn(move || {
        let reader = BufReader::new(read_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("{}", msg),
                Err(_) => break,
            }
        }
    });

    let stdin = io::stdin();
    let mut handle = stream;

    for line in stdin.lock().lines() {
        let message = line?;
        handle.write_all(message.as_bytes())?;
        handle.write_all(b"\n")?;
    }

    Ok(())
}
