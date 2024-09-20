fn raw_pointer_example() {
    let x = 42;
    let ptr = &x as *const i32;

    unsafe {
        println!("Value of x through raw pointer: {}", *ptr);
    }
}

fn main() {
    raw_pointer_example();
}
