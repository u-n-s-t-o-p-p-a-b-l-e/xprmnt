fn main() {
    let x = Box::new(10);
    let raw_x: *const i32 = &*x;

    unsafe {
        println!("Value through raw pointer: {}", *raw_x);
    }

    let mut y = 20;
    let raw_y: *mut i32 = &mut y;

    unsafe {
        *raw_y += 1;
        println!("Modified value through raw pointer: {}", *raw_y);
    }
}
