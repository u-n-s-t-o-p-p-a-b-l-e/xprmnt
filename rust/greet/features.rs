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


    impl Person {
        fn new(name: &str, age: u32) -> Person {
            Person {
                name: name.to_string(),
                age,
            }
        }

        fn introduce(&self) {
            println!("Hi, I'm {} and I'm {} years old.", self.name, self.age);
        }
    }

    let person = Person::new("Alice", 30);
    person.introduce();
}
