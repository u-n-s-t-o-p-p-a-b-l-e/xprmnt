use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use rand::Rng;

fn producer(tx: mpsc::Sender<i32>) {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let num = rng.gen_range(1..101);
        tx.send(num).unwrap();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..1001)));
    }
}

fn consumer(rx: mpsc::Receiver<i32>) {
    for num in rx {
        println!("Consumed: {}", num);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    let tx_producer = Arc::clone(&tx);
    let producer_handle = thread::spawn(move || {
        producer(tx_producer.lock().unwrap().clone());
    });

    let consumer_handle = thread::spawn(move || {
        consumer(rx);
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
