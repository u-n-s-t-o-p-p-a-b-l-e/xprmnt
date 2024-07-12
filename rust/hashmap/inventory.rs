use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();

    add_item(&mut inventory, "Apple", 10);
    add_item(&mut inventory, "Banana", 5);
    add_item(&mut inventory, "Orange", 8);

    remove_item(&mut inventory, "Banana", 2);

    update_item(&mut inventory, "Apple", 15);

    display_inventory(&inventory);
}
