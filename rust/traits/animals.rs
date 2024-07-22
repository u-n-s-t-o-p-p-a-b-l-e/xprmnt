trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn spead(&self) -> String {
        "woof!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn spead(&self) -> String {
        "Meow!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

fn print_animal_details(animal: &dyn Animal) {
    println!("{} says: {}", animal.name(), animal.speak());
}

fn main() {
    let dog = Dog { name: "Buddy".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    println!("Dog:");
    print_animal_details(&dog);

    println!("\nCat:");
    print_animal_details(&cat);
}
