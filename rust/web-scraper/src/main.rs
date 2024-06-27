use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::task;
use reqwest;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <output_directory> <url1> <url2> ... <urlN>", args[0]);
        std::process::exit(1);
    }

    let output_directory = &args[1];
    let urls: Vec<String> = args[2..].to_vec();

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for (index, url) in urls.iter().enumerate() {
        let results = Arc::clone(&results);
        let url = url.clone();
        let output_directory = output_directory.clone();

        let handle = task::spawn(async move {
            match fetch_and_save(&url, &output_directory, index).await {
                Ok(filename) => {
                    let mut results = results.lock().unwrap();
                    results.push(format!("saved {} to {}", url, filename));
                }
                Err(e) => {
                    let mut results = results.lock().unwrap();
                    results.push(format!("Failed to fetch {}: {}", url, e));
                }
            }
        });

        handles.push(handle);
    }
}
