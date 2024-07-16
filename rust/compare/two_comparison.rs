fn main() {
    let s1 = "this";
    let s2 = "that";

    let iter1 = s1.chars();
    let iter2 = s2.chars();

    while let (Some(c1), Some(c2)) = (iter1.next(), iter2.next()) {
        if c1 == c2 {
            println!("Same char: {}", c2);
        } else {
            println!("Different char: {} and {}", c1, c2);
        }
    }
}
