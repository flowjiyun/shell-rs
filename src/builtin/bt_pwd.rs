use std::env;

pub fn bt_pwd() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("{}", current_dir.display());
}