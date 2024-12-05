#[allow(unused_imports)]
use std::io::{self, Write};
use regex::Regex;

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();
    while stdin.read_line(&mut input).is_ok() {
        handle_command(input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn handle_command(command: &str) {
    if !is_valid(command) {
        println!("{}: command not found", command);
    }
}

fn is_valid(command: &str) -> bool {
    /// todo implement valid functions
    false
}