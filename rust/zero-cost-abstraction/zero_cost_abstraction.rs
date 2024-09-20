use std::mem;

fn main() {
    let mut old_value = String::from("Hey");
    let new_value = String::from("There");

    let replaced_value = mem::replace(&mut old_value, new_value);

    println!("Old value: {}", replaced_value);
    println!("Current value: {}", old_value);
}
