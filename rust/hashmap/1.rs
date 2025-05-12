// Encryption Decryption

use std::collections::HashMap;

fn main() {
    let mut enc = HashMap::new();
    enc.insert("a", "c");
    enc.insert("b", "d");
    enc.insert("c", "e");
    enc.insert("d", "f");
    enc.insert("e", "g");

    let enc_word = "ceg";
    let mut combined_result = String::with_capacity(enc_word.len());

    for char in enc_word.chars() {
        let target = &char;

        if let Some(key) = enc.iter()
            .find(|&(_, &val)| val == target.to_string().as_str())
                .map(|(&key, _)| key)
        {
            combined_result.push_str(key);
        }
    }
    println!("Combined decrypted result: {}", combined_result);
}
