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

    let domain = &args[1];
    match dns_lookup(domain) {
        Ok(addresses) => {
            println!("IP addresses for {}:", domain);
            for address in addresses {
                println!("{}", address);
            }
        }
        Err(e) => eprintln!("Error looking up {}: {}", domain, e);
    }
    Ok(())
}

fn dns_lookup(domain: &str) ->  io::Result<Vec<String>> {
    let mut addresses = Vec::new();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(domain).map_err(|e| io::Error::new::(io::ErrorKind::Other, e.to_string()))?;
    for addr in response.iter() {
        addresses.push(addr.to_string());
    }
    Ok(addresses)
}
