use std::cell::RefCell;

fn main() {
    let data = RefCell::new(42);

    *data.borrow_mut() += 1;
    println!("Data: {}", data.borrow());
}
