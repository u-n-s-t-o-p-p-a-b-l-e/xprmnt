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

    fn add_server(&mut self, server: Server) {
        self.servers.push_back(server);
    }

    fn distribute_request(&mut self, client_ip: &Ipv4Addr) {
        let server = &self.servers[self.current];
        server.handle_request(client_ip);

        self.current = (self.current + 1) % self.servers.len();
    }
}

fn main() {
    let servers_ips = vec![
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(192, 168, 1, 2),
        Ipv4Addr::new(192, 168, 1, 3),
        Ipv4Addr::new(192, 168, 1, 4),
    ];

    let mut load_balancer = LoadBalancer::new(
        server_ips
        .into_iter()
        .map(Server::new)
        .collet(),
    );

    let client_ips = vec![
        Ipv4Addr::new(203, 0, 113, 1),
        Ipv4Addr::new(203, 0, 113, 2),
        Ipv4Addr::new(203, 0, 113, 3),
        Ipv4Addr::new(203, 0, 113, 4),
        Ipv4Addr::new(203, 0, 113, 5),
    ];

    for client,ip in client_ips {
        load_balancer.distribute_request(&client_ip);
    }

    println!("Adding a new server with IP 192.168.1.5 ...");
    load_balancer.add_server(Server::new(Ipv4Addr::new(192, 168, 1, 5)));
}
