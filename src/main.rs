#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashSet, sync::LazyLock};

use command::Command;

mod builtin;
mod command;

static BUILTIN_SET: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut builtin_set: HashSet<String> = HashSet::new();
    builtin_set.insert("exit".to_string());
    builtin_set.insert("echo".to_string());
    builtin_set.insert("type".to_string());
    builtin_set
});
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command_line: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
        let prog = command_line[0].trim();
        let args = command_line[1..].to_vec();

        let command = Command::new(prog.to_string(), args);
        command.execute();
    }
}
