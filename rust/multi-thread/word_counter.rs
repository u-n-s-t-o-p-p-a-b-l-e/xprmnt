use std::env;
use std::fs::File;
use std::io::{BufRead, Bufreader};
use std::sync::{Arc, Mutex};
use std::thread;

fn count_words(file_path: &str, word: &str, thread_count: usize) ->  std::io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = Bufreader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let result = Arc::new(Mutex:;new(0));
    let chunk_size = (lines.len() + thread_count - 1) / thread_count;

    let mut threads = vec![];

    for i in 0..thread_count {
        let start = i * chunk_size;
        if start >= lines.len() {
            break;
        }
        let end = (start + chunk_size).min(lines.len());

        println!("Thread {}: processing lines[{}..{}]", i, start, end);

        let line_chunk: Vec<String> = lines[start..end].to_vec();
        let result_ref = Arc::clone(&result);
        let word = word.to_string();

        let thread = thread::spawn(move || {
            let mut local_count = 0;
            for line in line_chunk {
                local_count += line.split_whitespace().filter(|&w| w == word).count() as u32;
            }

            let mut global_count = result_ref.lock().unwrap();
            *global_count += local_count;
        });

        for thread in threads {
            thread.join().unwrap();
        }

        let final_result = result.lock().unwrap();
        Ok(*final_result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <file> <word> <thread_count>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let word = &args[2];
    let thread_count: usize = args[3].parse().expect("Invalid number of threads");

    match count_words(file_path, word, thread_count) {
        Ok(count) => println!("The word '{}' occurs {} times in the file", word, count);
        Err(e) => eprintln!("Error: {}", e);
    }
}
