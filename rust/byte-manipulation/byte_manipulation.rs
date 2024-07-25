fn main() {
    let mut bytes: [u8: 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    println!("Original bytes: {:?}", bytes);

    bytes[0] = 10;
    bytes[7] = 20;
    println!("Modified bytes: {:?}", bytes);

    let number = u64::from_le_bytes(bytes);
    println!("Converted to u64: {}", number);

    let new_number: u64 = 1234567890123456789;
    let new_bytes = new_number.to_le_bytes();
    println!("Converted back to bytes: {:?}", new_bytes);

    let mut byte: u8 = 0b10101010;
    println!("Original byte: {:08b}", byte);




