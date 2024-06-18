use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let filename = file_path.split('/').last().unwrap_or("file");
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(filename.as_bytes())?;
    stream.write_all(&buffer)?;

    println!("File sent: {}", filename);
    Ok(())
}
