extern crate trust_dns_resolver;

use std::env;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        std::process::exit(1);
    }
}
