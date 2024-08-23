use std::sync::mpsc;
use std::thread;

fn main() {
    let limit = 1000000;
    let num_threads = 8;

    let (tx, rx) = mpsc::channel();

    let chunk_size = limit / num_threads;

    for i in 0..num_threads {
        lt thread_tx = tx.clone();
        let start = i * chunck_size + 1;
        let end = if i == num_threads - 1 { limit } else { (i + 1) * chunk_size };

        thread::spawn(move || {
            for num in start..=end {
                if is_prime(num) {
                    thread_tx.send(num).unwrap();
                }
            }
        });
    }

    drop(tx);
}
