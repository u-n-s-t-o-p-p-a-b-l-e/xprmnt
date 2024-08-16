fn main() {
    let x = Box::new(10);
    let raw_x: *const i32 = &*x;

    unsafe {
        println!("Value through raw pointer: {}", *raw_x);
    }

}
