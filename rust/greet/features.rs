fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    println!("Sum of squares of even numbers: {}", sum);

    struct Person {
        name: String,
        age: u32,
    }
}
