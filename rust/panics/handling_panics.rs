use std::panic;

fn risky_code() {
    panic!("Something went wrong!");
}

fn main() {
    let result = panic::catch_unwind(|| {
        risky_code();
    });

    match result {
        Ok(_) => println!("Code executed successfully."),
        Err(_) => println!("A panic occured!"),
    }
}
