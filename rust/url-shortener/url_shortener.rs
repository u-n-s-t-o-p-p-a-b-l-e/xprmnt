use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

type UrlMap = Arc<Mutex<HashMap<String, String>>>;

const DATA_FILE: &str = "urls.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args]", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let url_map = Arc::new(Mutex::new(load_urls(DATA_FILE).unwrap_or_default()));

    match command.as_str() {
        "shorten" => {
            if args.len() != 3 {
                eprintln!("Usage: {} shorten <URL>", args[0]);
                std::process::exit(1);
            }
            let url = &args[2];
            let key = shorten_url(url, url_map.clone());
            if let Err(e) = save_url(&key, url, DATA_FILE) {
                eprintln!("Error saving URL: {}", e);
                std::process::exit(1);
            }
            println!("Shortened URL: {}", key);
        }
        "retrieve" => {
            if args.len() != 3 {
                eprintln!("Usage: {} retrieve <shortened-key>", args[0]);
                std::process::exit(1);
            }
            let key = &args[2];
            match retrieve_url(key, url_map.clone()) {
                Some(url) => println!("Original URL: {}", url),
                None => eprintln!("Key not found: {}", key),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

fn shorten_url(url: &str, url_map: UrlMap) ->  String {
    let key = generate_key();
    url_map.lock().unwrap().insert(key.clone(), url.to_string());
    key
}

fn retrieve_url(key: &str, url_map: UrlMap) ->  Option<String> {
    url_map.lock().unwrap().get(key).cloned()
}

fn load_urls(path: &str) ->  io::Result<HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut url_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, ' ');
        if let (Some(key), Some(url)) = (parts.next(), parts.next()) {
            url_map.insert(key.to_string(), url.to_string());
        }
    }

    Ok(url_map)
}

fn save_url(key: &str, url: &str, path: &str) ->  io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{} {}", key, url)?;
    Ok(())
}

fn generate_key() ->  String {
    use rand::{distributions::Alphanumeric, Rng};
    let key: String = rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect();
    key
}
