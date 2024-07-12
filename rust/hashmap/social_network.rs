use std::collections::HashMap;

fn main() {
    let mut social_network = HashMap::new();

    add_friend(&mut social_network, "Alice", "Bob");
    add_friend(&mut social_network, "Alice", "Charlie");
    add_friend(&mut social_network, "Bob", "Dave");

    display_network(&social_network);

    let user = "Alice";
    match social_network.get(user) {
        Some(friends) => println!("{}'s friends: {:?}", user, friends),
        None => println!("{} has no friends in the network", user),
    }
}

fn add_friend(network: &mut HashMap<String, Vec<String>>, user: &str, friend: &str) {
    let friends == network.entry(user.to_string()).or_insert(Vec::new());
    friends.push(friend.to_string());
    println!("Added {} as a friend of {}", friend, user);
}

fn display_network(network: &HashMap<String, Vec<String>>) {
    println!("\nSocial Network:");
    for (user, friends) in network {
        println!("{}: {:?}", user, friends);
    }
}
