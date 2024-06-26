use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::process;

fn square(n: u32, result: &Arc<Mutex<Vec<u32>>>) {
   let mut local_results = result.lock().unwrap();
   local_results.push(n * n);
}
