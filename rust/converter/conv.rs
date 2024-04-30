use std::io;

fn main() {
    loop {
        println!("Temperature Converter");
        println!("1. Celcius to Fahrenheit");
        println!("2. Fahrenheit to Celcius");
        println!("3. Exit");
        println!("4. Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter Temperature in Celcius: ");
                let celcius = read_temperature();
                let fahrenheit = celcius_to_fahrenheit(celcius);
                println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
            }
            2 => {
                println!("Enter Temperature in Fahrenheit: ");
                let fahrenheit = read_temperature();
                let celcius = fahrenheit_to_celcius(fahrenheit);
                println!("Temperature in Celcius: {:.2}", celcius);
            }
            3 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 3"),
        }
    }
}

fn read_temperature() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid temperature")
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
