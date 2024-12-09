mod lib;
use regex::Regex;
use std::fs::read_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env::*;
use lib::functions::echo::echo;
use crate::lib::functions::cd::cd;
use crate::lib::functions::exit::exit;
use crate::lib::functions::invalid_command::invalid_command;
use crate::lib::functions::pwd::pwd;
use crate::lib::functions::type_cmd::{find_binary, type_cmd};

const PROMPT: &str = "$ ";
const BUILTINS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

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
    let tokens = tokenize(input);
    let command = tokens[0].as_str(); // get the command
    let args: Vec<&str> = tokens[1..].iter().map(|s| s.as_str()).collect();


    // check if the command is a shell builtin
    for builtin in BUILTINS {
        if command == builtin {
            handle_builtin(&command, args);
            return;
        }
    }

    // check if the command is an executable found in the PATH and execute it
    if let Some(path) = find_binary(command) {
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
        "pwd"  => pwd(),
        "cd"   => cd(args.join(" ").as_str()),
        &_ => unreachable!() // error case if the command is not found in the builtins, should never happen
    }
}

/// Tokenize the input string into a vector of strings
/// each element of the vector is a token.
/// A token is a word separated by a space
/// except when the word is between quotes
fn tokenize(input: &str) -> Vec<String> {

    // Regex to match shell-style tokens
    // let re = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)"|'([^'\\]*(?:\\.[^'\\]*)*)'|(\S+)"#).unwrap();
    // let re = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)"|'([^'\\]*(?:\\.[^'\\]*)*)'|((?:\\.|[^\\\s])+)"#).unwrap();
    let re = Regex::new(r#""((?:\\.|[^"\\])*)"|'([^']*)'|((?:\\.|[^\\\s])+)"#).unwrap();



    let mut tokens = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(quoted_double) = cap.get(1) {
            // Double-quoted token
            tokens.push(unescape(quoted_double.as_str()).to_string());
        } else if let Some(quoted_single) = cap.get(2) {
            // Single-quoted token
            tokens.push(quoted_single.as_str().to_string());
        } else if let Some(unquoted) = cap.get(3) {
            // Unquoted token
            tokens.push(unescape(unquoted.as_str()));
        }
    }

    tokens
}


// Function to unescape backslash-escaped sequences
fn unescape(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            // Handle escaped character
            if let Some(next_char) = chars.next() {
                result.push(next_char);
            }
        } else {
            result.push(c);
        }
    }
    let test = result.split_ascii_whitespace().collect::<Vec<_>>().join("");
    test
}
