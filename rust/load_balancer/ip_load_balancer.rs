use std::net::{Ipv4Addr};
use std::Collections::VecDeque;
use std::io::{self, Write};

struct Server {
    ip_address: Ipv4Addr,
}

impl Server {
    fn new(ip: Ipv4Addr) -> Server {
        Server { ip_address: ip }
    }
    fn handle_request(&self, client_ip: &Ipv4Addr) {
        println!("Request from {} is being handled by server {}", client_ip, self.ip_address);
    }
}

struct LoadBalancer {
    servers: VecDeque<Server>,
    current: usize,
}

impl LoadBalancer {
    fn new(servers: Vec<Server>) -> LoadBalancer {
        LoadBalancer {
            servers: VecDeque::from(servers),
            current: 0,
        }
    }
}
