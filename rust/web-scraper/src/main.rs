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

    for handle in handles {
        handle.await.unwrap();
    }

    let results = results.lock().unwrap();
    for result in results.iter() {
        println!("{}", result);
    }
}

async fn fetch_and_save(url: &str, output_directory: &str, index: usize) ->  Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    let filename = format!("{}/{}.html", output_directory, index);
    let mut file = File::create(&filename).expect("Could not create file");
    file.write_all(response.as_bytes()).expect("Could not write to file");
    Ok(filename)
}
