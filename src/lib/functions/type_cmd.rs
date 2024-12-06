use std::env::{split_paths, var};
use std::io;
use std::io::Write;
use crate::{BUILTINS};

pub fn type_cmd(command: &str) {

    // check if the command is a shell builtin
    if BUILTINS.contains(&command) {
        println!("{} is a shell builtin", command);
        return;
    }

    let paths = find_command_in_path(command);

    if paths.len() > 0 {
        for path in paths {
            println!("{} is {}",command,  path.trim());
            io::stdout().flush().unwrap();
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
                Ok(e) => if e.file_name() == command {  found_paths.push(String::from(path.to_str().unwrap()) + "/" + command.trim()); },
                Err(_) => {}
            }
        }
    }
    found_paths
}