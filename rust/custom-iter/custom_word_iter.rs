struct WordIterator<'a> {
    text: &'a str,
    current_pos: usize,
}
