mod lib;

use std::fs::read_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env::*;
use crate::lib::functions::echo::echo;
use crate::lib::functions::exit::exit;
use crate::lib::functions::invalid_command::invalid_command;
use crate::lib::functions::type_cmd::type_cmd;

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


