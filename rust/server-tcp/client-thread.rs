use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server");

    loop {
        print!("Enter message ( or 'quit' to exit ): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Server close connection.");
            break;
        }

        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received response: {}", response);
    }

    Ok(())
}
