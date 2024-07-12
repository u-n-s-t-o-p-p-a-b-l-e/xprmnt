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
