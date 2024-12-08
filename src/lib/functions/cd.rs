use crate::lib::functions::pwd::pwd;

pub fn cd(input: &str) {
    if(input == "~" || input == "$HOME" || input.is_empty()) {
        let home = std::env::var("HOME").unwrap();
        std::env::set_current_dir(home).unwrap();
        return;
    }
    std::env::set_current_dir(input).unwrap_or_else(|c| eprintln!("cd: {}: {}", input, c));
}