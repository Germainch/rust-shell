use std::env::{split_paths, var};
use std::io;
use std::io::Write;
use std::process::Command;
use crate::{BUILTINS};


/// Print the type of command
pub fn type_cmd(command: &str) {

    // check if the command is a shell builtin
    if BUILTINS.contains(&command) {
        println!("{} is a shell builtin", command);
        return;
    }

    // check if the command is an executable found in the PATH
    if let Some(path) = find_binary(&command) {
        println!("{} is {}", command, path);
        return;
    }

    eprintln!("{}: not found", command);
}


/// Find the path of a command in the PATH environment variable
/// it returns the path of the command if found, None otherwise
/// this function is used by the type_cmd function and the handle_command function to execute the command
pub fn find_binary(command: &str) -> Option<String> {
    let line = var("PATH").unwrap();
    let paths = split_paths(&line);

    for path in paths {
        for entry in path.read_dir().expect("read_dir failed"){
            match entry {
                Ok(e) => if e.file_name() == command { return Some(path.to_str()?.to_string() + "/" + command); },
                Err(_) => {}
            }
        }
    }
    None
}