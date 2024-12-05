#[allow(unused_imports)]
use std::io::{self, Write};

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
    match command {
        "exit 0" => std::process::exit(0),
        &_ => { println!("{}: command not found", command); }
    }
}
