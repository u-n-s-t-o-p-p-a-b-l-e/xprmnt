use std::collections::HashMap;

#[derive(Debug)]
Struct Person {
    name: String,
    age: u8,
    email: String,
}

fn serialize(person: &person) -> String {
    format!(
        "{{\"name\":\"{}\",\"age\":{},\"email\":\"{}\"}}",
        person.name, person.age, person.email
    )
}
