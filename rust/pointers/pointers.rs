use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

type TextBuffer = Rc<RefCell<Vec<String>>>;

fn main() -> io::Result<()> {
    let buffer: TextBuffer = Rc::new(RefCell::new(Vec::new()));

    loop {
        print_buffer(&buffer)?;

        println!("Enter a line of text ('q' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "q" {
            break;
        } else if !input.is_empty() {
            add_line(&buffer, input);
        }
    }

    Ok(())
}

fn print_buffer(buffer: &TextBuffer) -> io::Result<()>{
   let borrowed_buffer = buffer.borrow();
   for line in borrowed_buffer.iter() {
       println!("{}", line);
   }
   Ok(())
}

fn add_line(buffer: &TextBuffer, line: &str) {
    let mut borrowed_buffer = buffer.borrow_mut();
    borrowed_buffer.push(line.to_string());
}
