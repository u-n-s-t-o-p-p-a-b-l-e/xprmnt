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
}
