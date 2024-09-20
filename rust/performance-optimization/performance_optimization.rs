use std::hint::black_box;

fn main() {
    let input = vec![1, 2, 3, 4, 5];

    let result = black_box(sum(&input));

    println!("Sum is: {}", result);
}

fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
