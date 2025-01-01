pub fn bt_cd(args: Vec<String>) {
    if let Some(dir) = args.get(0) {
        if dir == "~" {
            let home_dir = std::env::var("HOME").unwrap_or_default();
            if let Err(_e) = std::env::set_current_dir(&home_dir) {
                println!("cd: {}: No such file or directory", home_dir);
            }
            return;
        }
        if let Err(_e) = std::env::set_current_dir(dir) {
            println!("cd: {}: No such file or directory", dir);
        }
    }
}