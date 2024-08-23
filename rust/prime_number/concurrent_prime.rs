use std::sync::mpsc;
use std::thread;

fn main() {
    let limit = 1000000;
    let num_threads = 8;

    let (tx, rx) = mpsc::channel();

    let chunk_size = limit / num_threads;
}
