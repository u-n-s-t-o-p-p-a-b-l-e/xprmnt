use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(format!("{}\n", name).as_bytes())?;

    let stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        handle_read(stream_clone);
    });

    handle_write(stream);

    Ok(())
}

fn handle_read(mut stream: TcpStream) {
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        match line {
            Ok(message) => println!("{}", message),
            Err(_) => break,
        }
    }
}
