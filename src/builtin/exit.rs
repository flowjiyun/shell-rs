pub fn exit(args: Vec<&str>) {
    if let Some(exit_code) = args.get(0) {
        if let Ok(exit_code) = exit_code.parse::<i32>() {
            std::process::exit(exit_code);
        } else {
            std::process::exit(1);
        }
    } else {
        std::process::exit(0);
    }
}