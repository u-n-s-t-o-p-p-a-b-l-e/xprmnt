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
}
