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
    }
}