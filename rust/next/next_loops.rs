fn main() {
    let vec = vec![1, 2, 3];
    let mut iter = vec.iter();

    while let Some(value) = iter.next() {
        println!("Next value: {}", value);
    }
}
