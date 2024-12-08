pub fn pwd() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}