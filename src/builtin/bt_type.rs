use std::fs;

use crate::BUILTIN_SET;

pub fn bt_type(args: Vec<String>) {
    if let Some(prog) = args.get(0) {
        // first check if the prog is builtin
        if BUILTIN_SET.contains(prog) {
            println!("{} is a shell builtin", prog);
            return;
        }
        // then check if the prog is in the PATH
        let env_path = std::env::var("PATH").unwrap_or(String::default());
        let paths: Vec<&str> = env_path.split(":").collect();
        for path in paths {
            // check if the file exists
            let file_path = format!("{}/{}", path, prog);
            if fs::metadata(&file_path).is_ok() {
                println!("{} is {}", prog, file_path);
                return;
            }
        }
        println!("{}: not found", prog);
    }
}
