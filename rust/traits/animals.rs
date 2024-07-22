trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}
