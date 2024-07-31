use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let spinlock = Arc::new(SpinLock::new());
    let shared_data = Arc::new(AtomicI32::new(0));

    loop {
        print!("Enter the number of threads to spawn (0 to exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num_threads: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                continue;
            },
        };

        if num_threads == 0 {
            println!("Exiting...");
            break;
        }

        let mut handles = vec![];

        for _ in 0..num_threads {
            let spinlock_clone = Arc::clone(&spinlock);
            let shared_data_clone: Arc<AtomicI32> = Arc::clone(&shared_data);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    spinlock_clone.lock();

                    let val = shared_data_clone.load(Ordering::SeqCst);
                    shared_data_clone.store(val + 1, Ordering::SeqCst);

                    spinlock_clone.unlock();

                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
