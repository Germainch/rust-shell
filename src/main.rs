use std::fs::read_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env::*;

const PROMPT: &str = "$ ";
const BUILTINS: [&str; 3] = ["echo", "type", "exit"];

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

    // split the line received into a vector, with the command in the index 0
    let tokens = input.split_whitespace().collect::<Vec<_>>();
    let command = tokens[0]; // get the command
    let args = tokens[1..].to_vec();

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


/// the type command is used to determine the type of command
fn type_cmd(command: &str) {

    // check if the command is a shell builtin
    if BUILTINS.contains(&command) {
        println!("{} is a shell builtin", command);
        return;
    }

    let paths = find_command_in_path(command);

    if paths.len() > 0 {
        for path in paths {
            println!("{}", path);
        }
        return;
    }

    eprintln!("{}: command not found", command);
}

fn find_command_in_path(command: &str) -> Vec<String> {
    let line = var("PATH").unwrap();
    let paths = split_paths(&line);
    let mut found_paths = Vec::new();

    for path in paths {
        for entry in path.read_dir().expect("read_dir failed"){
            match entry {
                Ok(e) => if e.file_name() == command {  found_paths.push(String::from(path.to_str().unwrap()) + "/" + command); },
                Err(_) => {}
            }
        }
    }
    found_paths
}
