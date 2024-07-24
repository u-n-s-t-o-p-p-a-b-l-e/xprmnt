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

fn deserialize(json: &str) -> Result<Person, &'static str> {
    let mut map: HashMap<&str, &str> = HashMap::new();

    let json = json.trim_matches(|c| c == '{' || c == '}');
}
