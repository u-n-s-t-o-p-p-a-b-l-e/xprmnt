use std::sync::Once;

static INIT: Once = Once::new();

fn initialize() {
    println!("This runs only once!");
}

fn main() {
    INIT.call_once(|| initialize());
    INIT.call_once(|| initialize()); // Won't run again
}
