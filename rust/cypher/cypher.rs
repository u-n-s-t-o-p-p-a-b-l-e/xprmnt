use std::fs;
use std::io::{self, Read, Write};

fn main() {
    loop {
        println!("1. Encrypt file");
        println!("2. Decrypt file");
        println!("3. Exit");
        println!("4. Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                continue;
            }
        };

        match choice {
            1 => encrypt_file(),
            2 => decrypt_file(),
            3 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 3"),
        }
    }
}

fn encrypt_file() {
    println!("Enter the name of the file to encrypt: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read input");
    let filename = filename.trim();

    let content = read_file_content(filename).expect("Failed to read file");
    let encrypted_content = caesar_cipher(&content, 3);
    write_file_content(&format!("{}.enc", filename), &encrypted_content).expect("Failed to write encrypted content to file");

    println!("file encrypted successfully");
}

fn decrypt_file() {
    println!("Enter the name of the file to decrypt");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read input");
    let filename = filename.trim();

    let content = read_file_content(filename).expect("Failed to read file");
    let decrypted_content = caesar_cipher(&content, -3);
    write_file_content(&format!("{}.dec", filename), &decrypted_content).expect("Failed to write decrypted content to file"); 

    println!("File decrypted successfully");
}

fn read_file_content(filename: &str) -> io::Result<String> {
    let mut file = fs::File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file_content(filename: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {b'a' } else {b'A' };
                let offset = (c as i32 - base as i32 + shift + 26) % 26;
                (offset + base as i32) as u8 as char
            } else {
                c
            }
        })
    .collect()
}
