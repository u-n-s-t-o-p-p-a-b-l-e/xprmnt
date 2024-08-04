use std::io::{self, Read, Write};
use std::fs::File;

const MAX_MATCH: usize = 258;
const MIN_MATCH: usize = 3;

fn main() -> io::Result<()> {
    let data = b"Example data to compress using the Deflate algorithm in Rust.";

    let compressed_data = deflate_compress(data)?;

    let mut output_file = File::create("compressed.deflate")?;
    output_file.write_all(&compressed_data)?;

    println!("Data compressed and written to compressed.deflate");

    let mut input_file = File::open("compressed.deflate")?;
    let mut compressed_data = Vec::new();
    input_file.read_to_end(&mut compressed_data)?;
    
    let decompressed_data = deflate_decompress(&compressed_data)?;

    println!("Decompressed data: {}", String::from_utf8_lossy(&decompressed_data));

    Ok(())
}

fn deflate_compress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = DeflateEncoder::new(Vec::new());
    encoder.write_all(data)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

fn deflate_decompress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

struct DeflateEncoder<W: Write> {
    inner: W,
}

impl<W: Write> DeflateEncoder<W> {
    fn new(inner: W) -> Self {
        DeflateEncoder { inner }
    }

    fn finish(self) -> io::Result<W> {
        Ok(self.inner)
    }
}

impl<W: Write> Write for DeflateEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}


struct DeflateDecoder<'a> {
    inner: &'a [u8],
}

impl<'a> DeflateDecoder<'a> {
    fn new(inner: &'a [u8]) -> Self {
        DeflateDecoder { inner }
    }
}

impl<'a> Read for DeflateDecoder<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&mut self.inner).read(buf)
    }
}
