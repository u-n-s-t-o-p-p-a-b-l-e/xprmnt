use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::process;

fn square(n: u32, result: &Arc<Mutex<Vec<u32>>>) {
   let mut local_results = result.lock().unwrap();
   local_results.push(n * n);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} < numbers separated by spaces>", args[0]);
        process::exit(1);
    }

    let numbers: Vec<u32> = args[1..]
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let result = Arc::new(Mutex::new(vec![]));
    let thread_count = 4;
    let chunk_size = (numbers.len() + thread_count - 1) / thread_count;

    let mut threads = vec![];
}
