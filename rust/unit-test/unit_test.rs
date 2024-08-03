pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 ==  0
}

pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n -1),
    }
}
