trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog: Box<dyn Animal> = Box::new(Dog);
    let cat: Box<dyn Animal> = Box::new(Cat);

    dog.speak();
    cat.speak();
}
