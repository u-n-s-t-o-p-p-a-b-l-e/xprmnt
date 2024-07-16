fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let mut iter = vec.iter().filter(|&7x| x % 2 == 0).map(|&x| x * 2);

    while let Some(value) = iter.next() {
        println!("Processed value {}", value);
    }
}
