use std::sync::{Arc, Mutex};

struct LoadBalancer {
    servers: Vec<String>,
    index: Mutex<usize>,
}

impl LoadBalancer {
    fn new(servers: Vec<String>) -> Arc<Self> {
        Arc::new(LoadBalancer {
            servers,
            index: Mutex::new(0),
        })
    }

    fn get_server(&self) -> String {
        let mut idx = self.index.lock().unwrap();
        let server = self.servers[*idx].clone();
        *idx = (*idx + 1) % self.servers.len();
        server
    }
}

fn main() {
    let servers = vec![
        "http://server1.example.com".to_string(),
        "http://server2.example.com".to_string(),
        "http://server3.example.com".to_string(),
    ];

    let load_balancer = LoadBalancer::new(servers);

    for _ in 0..10 {
        let server = load_balancer.get_server();
        println!("Request forwarded to: {}", server);
    }
}
