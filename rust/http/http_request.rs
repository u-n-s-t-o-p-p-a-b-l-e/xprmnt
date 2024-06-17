use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    match fetch_url(url) {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn fetch_url(url: &str) ->  Result<String, Box<dyn std::error::Error>> {
    let (host, path) = parse_url(url)?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host
        );

    let mut stream = TcpStream::connect(format!("{}:80", host));
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let body = extract_body(&response)?;
    Ok(body.to_string())
}

fn parse_url(url: &str) ->  Result<(&str, &str), Box<dyn std::error::Error>> {
    let mut parts = url.splitn(2, '/');
    let host = parts.next().ok_or("Invalid URL")?;
    let path = parts.next().unwrap_or("/");
    Ok((host, path))
}

fn extract_body(response: &str) ->  Result<&str, Box<dyn std::error::Error>> { 
    let parts: Vec<&str> = response.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err("Invalid HTTP response".into());
    }
    Ok(parts[1])
}
