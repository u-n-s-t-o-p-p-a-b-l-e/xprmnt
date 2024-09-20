use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                let message = format!("Producer {}: Message {} ", i, j);
                tx_clone.send(message).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}
