use std::{collections::HashMap, fs::File, io::Write};

pub fn bt_echo(args: Vec<String>, file_map: &mut HashMap<String, File>) {
    let mut output = String::new();
    for arg in args {
        output.push_str(&arg);
        output.push(' ');
    }
    let output = output.trim();

    if !file_map.contains_key("1") {
        println!("{}", output);
        return;
    }

    for (fd, file) in file_map.iter_mut() {
        if fd == "1" {
            if let Err(e) = writeln!(file, "{}", output) {
                eprintln!("{}", e);
            }
        }
    }
}