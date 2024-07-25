fn main() {
    let mut bytes: [u8: 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    println!("Original bytes: {:?}", bytes);

    bytes[0] = 10;
    bytes[7] = 20;
    println!("Modified bytes: {:?}", bytes);
}




