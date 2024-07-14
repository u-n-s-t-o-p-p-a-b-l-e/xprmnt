fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1: String = String::from("long string is long");
    let string2: &str ="xyz";

    let result: &str = longest(x: string1.as_str(), y: string2);
    println!("The longest string is {}", result);
}
