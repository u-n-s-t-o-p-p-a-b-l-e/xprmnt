fn main() {
    let vec = vec![10, 20, 30];
    let mut iter = vec.iter();

    let first = iter.next();
    let second = iter.next();
    let third = iter.next();
    let fourth = iter.next();

    println!("First:  {:?}", first);
    println!("Second: {:?}", second);
    println!("Third:  {:?}", third);
    println!("Fourth: {:?}", fourth);
}
