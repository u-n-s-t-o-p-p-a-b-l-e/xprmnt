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

    for pair in json.split(',') {
        let mut iter = pair.split(':');
        let key = iter.next().ok_or("Invalid JSON format")?.trim_matches('"');
        let value = iter.next().ok_or("Invalid JSON format")?.trim_matches('"');
        map.insert(key, value);
    }
}
