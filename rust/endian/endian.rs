fn main() {
    let value: u32 = 0x12345678;

    if cfg!(target_endian = "little") {
        println!("System is little-endian.");
    } else {
        println!("System is big-endian.");
    }

    let little_endian = value.to_le();
    println!("Little-endian representation: 0x{:x}", little_endian);

    let big_endian = value.to_be();
    println!("Big-endian representation: 0x{:x}", big_endian);

    if little_endian == value {
        println!("Value is stored in little-endian format.");
    } else {
        println!("Value is stored in big-endian format.");
    }

    let manual_big_endian = manual_to_big_endian(value);
    println!("Manually converted to big-endian: 0x{:x}", manual_big_endian);

    let manual_little_endian = manual_to_little_endian(value);
    println!("Manually converted back to little-endian: 0x{:x}", manual_little_endian);
}

fn manual_to_big_endian(value: u32) -> u32 {
    value.swap_bytes()
}

fn manual_to_little_endian(value: u32) -> u32 {
    value.swap_bytes()
}
