struct CharIterator<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> CharIterator<'a> {
    fn new(text: &'a str) -> Self {
        CharIterator { text, position: 0 }
    }
}

impl<'a> Iterator for CharIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.text.len() {
            let ch = self.text[self.position..].chars().next()?;
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }
}

fn main() {
    let text = "Hey, there!";
    let mut iterator = CharIterator::new(text);

    while let Some(ch) = iterator.next() {
        println!("{}", ch);
    }
}


