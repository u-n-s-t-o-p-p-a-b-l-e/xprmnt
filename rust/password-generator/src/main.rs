use rand::Rng;
use std::io;

fn main() {
    println!("Password Generator");

    let length = get_password_length();

    let password = generate_password(length);
    println!("Generated password: {}", password);
}

fn get_password_length() -> usize {
    loop {
        println!("Enter the length of the password:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize() {
            Ok(length) if length > 0 => return length,
            _ = println!("Please enter a valid positive integer"),
        }
    }
}

fn generate_password(length: usize)   ->  String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";

    let mut rng = rand::thread_rng();
    let password: String = (0..length).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    })
    .collect();

    password
}
