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
