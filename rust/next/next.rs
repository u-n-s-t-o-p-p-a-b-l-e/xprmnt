fn main() {
    let vec = vec![1, 2, 3];
    let mut iter = vec.iter();

    if let Some(first) = iter.next() {
        println!("First element: {}", first);
    }
    if let Some(second) = iter.next() {
        println!("Second element: {}", second);
    }
    if let Some(third) = iter.next() {
        println!("Third element: {}", third);
    }
    if iter.next().is_none() {
        println!("No more elements");
    }
}
