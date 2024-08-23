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

    let mut primes = Vec::new();

    for prime in rx {
        primes.push(prime);
    }

    primes.sort();
    println!("Found {} primes up to {}: {:?}", primes.len(), limit, &pprimes[0..10]);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 ==  0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
