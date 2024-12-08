mod lib;

use std::fs::read_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env::*;
use lib::functions::echo::echo;
use crate::lib::functions::exit::exit;
use crate::lib::functions::invalid_command::invalid_command;
use crate::lib::functions::type_cmd::{find_in_path, type_cmd};

const PROMPT: &str = "$ ";
const BUILTINS: [&str; 3] = ["echo", "type", "exit"];

/// main function
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

/// Handle the command received from the user
/// first checks in the builtins commands and then in the PATH
/// if the command is not found, it prints an error message
fn handle_command(input: &str) {

    // split the line received into a vector, with the command in the index 0
    let tokens = input.split_whitespace().collect::<Vec<_>>();
    let command = tokens[0]; // get the command
    let args = tokens[1..].to_vec();

    // check if the command is a shell builtin
    for builtin in BUILTINS {
        if command == builtin {
            return handle_builtin(command, args);
        }
    }

    // check if the command is an executable found in the PATH and execute it
    if let Some(path) = find_in_path(command) {
        std::process::Command::new(path).args(args).status().expect("failed to execute process");
        return;
    }

    // if the command is not found, print an error message
    eprintln!("{}: not found", command);

}

/// Handle the builtin commands
fn handle_builtin(command: &str, args: Vec<&str>) {
    match command.trim().to_lowercase().as_str() {
        "exit" => exit(),
        "echo" => echo(args.join(" ").as_str()),
        "type" => type_cmd(args.join(" ").as_str()),
        &_ => invalid_command(command) // error case if the command is not found in the builtins, should never happen
    }
}


