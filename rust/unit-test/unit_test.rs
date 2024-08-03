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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, -3), -5);
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
        assert!(is_even(0));
    }
}
