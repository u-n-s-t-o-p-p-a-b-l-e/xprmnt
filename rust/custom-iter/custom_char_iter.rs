struct Chariterator<'a> {
    text: &'str,
    current_pos: usize,
}

impl<'a> CharIterator<'a> {
    fn new(text: &'a str) -> CharIterator<'a> {
        CharIterator { text, current_post: 0 }
    }
}
