trait Animal {
    fn speak(&self)
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Meow!");
    }
}
