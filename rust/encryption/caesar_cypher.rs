fn caesar_cipher(text: &str, shift: i8) -> String {
    let shift = (shift % 26 + 26) % 26;

    let mut result = String::new();

    for c in text.chars() {
        let shifted_char = if c.is_ascii_alphabetic() {
            let first_char = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as u8 - first_char + shift as u8) % 26) + first_char;
            shifted as char
        } else {
            c
        };
        result.push(shifted_char);
    }

    result
}

fn main() {
    let plaintext = "Hello, Rust!";
    let shift = 3;

    let ciphertext = caesar_cipher(plaintext, shift);
    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);

    let decrypted = caesar_cipher(&ciphertext, -shift);
    println!("Decrypted: {}", decrypted);
}
