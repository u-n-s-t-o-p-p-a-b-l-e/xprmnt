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
            let key Err(e) = save_url(&key, url, DATA_FILE) {
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
