use std::fmt::Display;

trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

fn print_items<T: Printable>(items: &[T]) {
    for item in items {
        item.print();
    }
}

fn main() {
    let numbers = vec![1, 2, 3];
    print_items(&numbers);

    let words = vec!["Hi", "world"];
    print_items(&words);
}
