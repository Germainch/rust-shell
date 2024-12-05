#[allow(unused_imports)]
use std::io::{self, Write};
use regex::Regex;

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    if is_invalid(command) {
        println!("{}: command not found", command);
    }
}

fn is_invalid(command: &str) -> bool {
    let re = regex::Regex::new(r"invalid_command.*").unwrap();
    re.is_match(command)
}