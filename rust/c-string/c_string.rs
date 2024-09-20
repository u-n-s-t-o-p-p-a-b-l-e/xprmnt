use std::ffi::CString;

fn main() {
    let c_string = CString::new("Hey, there!").expect("CString::new failed");

    let c_str = c_string.as_c_str();
    let rust_str = c_str.to_str().expect("Failed to convert back to Rust string");

    println!("C string: {:?}", c_str);
    println!("Rust string: {}", rust_str);
}
