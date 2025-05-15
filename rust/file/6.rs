use std::fs::{self, File};
use std::io::{self, BufWriter, BufRead, BufReader, Write};

fn remove_line (
    filename: &str,
    line_num: usize
) -> io::Result<()> {
    let temp_filename = format!("{}.tmp", filename);
    let temp_file = File::create(&temp_filename)?;
    let mut temp_writer = BufWriter::new(temp_file);

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i != line_num-1 {
            writeln!(temp_writer, "{}", line?)?;
        }
    }

    fs::rename(temp_filename, filename)?;
    Ok(())
}

fn main() -> io::Result<()> {
    remove_line("progress.txt", 2)?;
    Ok(())
}


