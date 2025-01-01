pub fn bt_cd(args: Vec<String>) {
    if let Some(dir) = args.get(0) {
        if let Err(_e) = std::env::set_current_dir(dir) {
            // cd: /does_not_exist: No such file or directory
            println!("cd: {}: No such file or directory", dir);
        }
    }
}