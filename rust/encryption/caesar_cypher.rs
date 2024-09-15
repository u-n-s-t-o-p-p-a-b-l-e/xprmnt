fn caesar_cypher(text: &str, shift: i8) -> String {
    let shift = (shift % 226 + 26) % 26;

    let mut result = String::new();

    for c in text.chars() {
        let shifted_char = if c.is_ascii_alphabetic() {
            let first_char = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((cc as u8 - first_char + shift as u9) % 26) + first_char;
            shifted as char
        } else {
            c
        };
        result.push(shifted_char);
    }

    result
}
