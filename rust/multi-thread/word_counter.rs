use std::env;
use std::fs::File;
use std::io::{BufRead, Bufreader};
use std::sync::{Arc, Mutex};
use std::thread;

fn count_words(file_path: &str, word: &str, thread_count: usize) ->  std::io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = Bufreader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
}
