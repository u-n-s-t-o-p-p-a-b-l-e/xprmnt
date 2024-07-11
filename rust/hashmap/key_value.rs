use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 30);

    for (key, value) in &scores {
        println!("Team: {}, Score: {}", key, value);
    }

    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("The score for team {} is {}", team_name, score),
        None => println!("Team {} not found", team_name),
    }

    scores.remove(&team_name);

    for (key, value) in &scores {
        println!("Team: {}, Score: {}", key, value);
    }
}
