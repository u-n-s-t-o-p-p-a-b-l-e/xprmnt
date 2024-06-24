use std::env;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::io::{self, Write};;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() ! = 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        std::process::exit(1);
    }

    let domain = &args[1];
    match dns_lookup(domain) {
        Ok(addresses) => {
            println!("IP addresses for {}:" domain);
            for address in addresses {
                println!("{}", address);
            }
        }
        Err(e) => eprintln!("Error looking up {}: {}", domain, e),
    }

    Ok(())
}

fn dns_lookup(domain: &str) ->  io::Result<Vec<String>> {
    let mut addresses = Vec::new();
    let query = format!("{}:53", domain);
    match query.to_socket_address() {
        Ok(addrs) => {
            for addr in addrs {
                addresses.push(addr.to_string());
            }
            Ok(addresses)
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}
