pub fn bt_exit(args: Vec<String>) {
    if let Some(exit_code) = args.get(0) {
        match exit_code.parse::<i32>() {
            Ok(code) => std::process::exit(code),
            Err(_) => std::process::exit(0),
        }
    } else {
        std::process::exit(0);
    }
}