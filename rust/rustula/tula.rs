use std::fs;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let source_path = "./inc.tula";
    let source = fs::read_to_string(source_path).map_err(|err| {
        eprintln!("ERROR: could not read file {source_path}: {err}");
    })?;

    let tokens: Vec<&str> = source.split(&[' ', '\n']).collect();
    let mut lexer: &[&str] = &tokens;

    println!("{head}", head = lexer[0]);

    println!("{lexer:?}");
    lexer = &lexer[1..];
    println!("{lexer:?}");

    Ok(())
}

// define file path
// read file content and map error
// define error type
// token splitter
// print the result
// define lexer
// remove tokens from the left
// print for debugging
