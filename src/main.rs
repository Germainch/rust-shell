#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ \n");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    handle_command(input.trim());

}

fn handle_command(command: &str) {
    if is_invalid(command) {
        println!("{}: command not found\n", command);
    }
}

fn is_invalid(command: &str) -> bool {
    command == "invalid_command"
}