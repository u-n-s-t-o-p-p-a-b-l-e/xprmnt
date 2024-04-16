use std::fs;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let source_path = "./inc.tula";
    let source = fs::read_to_string(source_path).map_err(|err| {
        eprintln!("ERROR: could not read file {source_path}: {err}");
    })?;

    let tokens: Vec<_> = source.split(&[' ', '\n']).collect();

    println!("{tokens:?}");

    Ok(())
}
