use std::io::{self, Read, Write};
use std::fs::File;

const MAX_MATCH: usize = 258;
const MIN_MATCH: usize = 3;

fn main() -> io::Result<()> {
    let data = b"Example data to compress using the Deflate algorithm in Rust.";

    let compressed_data = deflate_compress(data)?;

    let mut output_file = File::create("compressed.deflatte")?;
    output_file.write_all(&compressed_data)?;

    println!("Data compressed and written to compressed.deflate");

    let mut input_file = File::open("compressed.deflate")?;
    let mut compressed_data = Vec::new();
    input_file.read_to_end(&mut compressed_data)?;
    
    let decompressed_data = deflate_decompress(&compressed_data)?;

    println!("Decompressed data: {}", String::from_utf8_lossy(&decompressed_data));

    Ok(())
}






















