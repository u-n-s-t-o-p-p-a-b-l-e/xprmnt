fn main() {
    let mut x: i32 = 42;
    let r1: *const i32 = &x;
    let r2: *mut i32 = &mut x;

    unsafe {
        println!("r1 points to: {}", *r1);
        println!("r1 points to: {}", *r2);

        *r2 = 100;
        println!("Now, r2 points to: {}", *r2);
    }

    println!("Final value of x: {}", x);
}
