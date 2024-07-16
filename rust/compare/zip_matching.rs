fn main() {
    let s1 = "there";
    let s2 = "thxre";

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            println!("Matching character: {}", c1);
        } else {
            println!("Different characters: {} and {}", c1, c2);
        }
    }
}
