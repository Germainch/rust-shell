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

fn handle_command(input: &str) {

    /// split the line received into a vector, with the command in the index 0
    let split = input.split_whitespace().collect::<Vec<_>>();
    let command = split[0];
    let args = split[1..].to_vec();

    match command.trim().to_lowercase().as_str() {
        "exit" => exit(),
        "echo" => echo(args.join(" ").as_str()),
        "type" => type_cmd(args.join(" ").as_str()),
        &_ => invalid_command(command)
    }
}

fn exit(){
    std::process::exit(0);
}

fn echo(arg: &str) {
    println!("{}", arg);
}

fn invalid_command(command: &str) {
    eprintln!("{}: command not found", command);
}

fn type_cmd(command: &str) {
    match command {
        "type" | "echo" | "exit"  => println!("{} is a shell builtin", command),
        &_ => {println!("{}: not found", command);}
    }
}