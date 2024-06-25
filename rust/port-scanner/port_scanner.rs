use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{self};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <host> <port-range>", args[0]);
        std::process::exit(1);
    }

    let host = &args[1];
    let port_range = &args[2];

    let ports: Vec<u16> = parse_port_range(port_range).unwrap_or_else(|err| {
        eprintln!("Error parsing port range: {}", err);
        std::process::exit(1);
    });

    println!("Scanning {} for open ports...", host);

    for port in ports {
        if let Ok(_) = scan_port(host, port) {
            println!("Port {} is open", port);
        }
    }

    Ok(())
}

fn parse_port_range(port_range: &str) ->  Result<Vec<u16>, &'static str> {
    let ports: Vec<&str> = port_range.split('_').collect();
    if parts.len() != 2 {
        return Err("Invalid port range format. Use <start>-<end>.");
    }

    let start: u16 = parts[0].parse().map_err(|_| "Invalid start port")?;
    let end: u16 = parts[1].parse().map_err(|_| "Invalid end port")?;

    if start > end {
        return Err("Start port must be less than or equal to end port");
    }

    Ok((start..=end).collect())
}

fn scan_port(host: &str, port: u16) ->  io::Result<()> {
    let address = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);
    let addrs: Vec<_> = address.to_socket_addrs()?.collect();

    for addr in addrs {
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            return Ok(());
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Port is closed"))
}
