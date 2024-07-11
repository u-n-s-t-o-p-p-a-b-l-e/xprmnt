use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(k: String::from("Blue"), v: 10);
    scores.insert(k: String::from("Yellow"), v: 50);
    scores.insert(k: String::from("Green"), v: 30);

    for (key: &String, value: &i32) in &scores {
        println!("Team: {}, Score: {}", key, value);
    }

    let team_name: String = String::from("Blue");
    match scores.get(&team_name) {
        Some(score: &i32) => println!("The score for team {} is {}", team_name, score),
        None => println!("Team {} not found", team_name),
    }

    scores.remove(&team_name);

    for (key: &String, value: &i32) in &scores {
        println!("Team: {}, Score: {}", key, value);
    }
} fn main
