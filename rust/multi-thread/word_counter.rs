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
    }
}
