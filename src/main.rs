#[allow(unused_imports)]
use std::io::{self, Write};

use builtin::{bt_echo::bt_echo, bt_exit::bt_exit, bt_type::bt_type};

mod builtin;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command_line = input.trim().split(" ").collect::<Vec<&str>>();
        let builtin = command_line[0].trim();
        let args = command_line[1..].to_vec();
        match builtin {
            "exit" => {
                bt_exit(args);
            },
            "echo" => {
                bt_echo(args);
            },
            "type" => {
                bt_type(args);
            }
            _ => {
                println!("{}: command not found", builtin);
            }
        }
    }
}
