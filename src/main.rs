#[allow(unused_imports)]
use std::io::{self, Write};

use builtin::exit::exit;

mod builtin;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command_line = input.split(" ").collect::<Vec<&str>>();
        let builtin = command_line[0].trim();
        match builtin {
            "exit" => {
                let code = if command_line.len() > 1 {
                    command_line[1].trim().parse::<i32>().unwrap()
                } else {
                    0
                };
                exit(code);
            }
            _ => {
                println!("{}: command not found", builtin);
            }
        }
    }
}
