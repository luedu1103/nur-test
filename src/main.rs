use std::io::{self, Read};

fn print_shit(input: &str) {
 if input.is_empty() {
        println!("No input provided.");
    } else {
        println!("Hello from Nur!: {}", input);
    }
}

fn main() {
    // Read from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Process input
    let input = input.trim();
    print_shit(input);
}

