struct WordIterator<'a> {
    text: &'a str,
    current_pos: usize,
}

impl<'a> WordIterator<'a> {
    fn new(text: &'a str) -> WordIterator<'a> {
        WordIterator { text, current_pos: 0 }
    }
}
