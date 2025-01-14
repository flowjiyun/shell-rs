#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::{HashMap, HashSet}, fs::File, sync::LazyLock};

use command::Command;
use tokenizer::Tokenizer;

mod builtin;
mod command;
mod tokenizer;

static BUILTIN_SET: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut builtin_set: HashSet<String> = HashSet::new();
    builtin_set.insert("exit".to_string());
    builtin_set.insert("echo".to_string());
    builtin_set.insert("type".to_string());
    builtin_set.insert("pwd".to_string());
    builtin_set.insert("cd".to_string());
    builtin_set
});
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let tokens = preprocess_tokenize(input.trim());
        if tokens.is_empty() {
            continue;
        }

        let mut command_line: Vec<String> = Vec::new();
        let mut redirections: Vec<(String, String)> = Vec::new();

        preprocess_redirection(&tokens, &mut command_line, &mut redirections);
        // println!("{:?}", command_line);        
        // println!("{:?}", redirections);
        let file_map = process_redirection(&redirections);
        let prog = command_line[0].trim();
        let args = command_line[1..].to_vec();

        let mut  command = Command::new(prog.to_string(), args, file_map);
        command.execute();
    }
}

fn preprocess_tokenize(input: &str) -> Vec<String> {
    Tokenizer::tokenize(input)
}

fn preprocess_redirection(tokens: &Vec<String>, command_line: &mut Vec<String>, redirections: &mut Vec<(String, String)>) {
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        if token == ">" || token == "1>" || token == "2>" {
            match tokens.peek() {
                Some(&next_token) => {
                    redirections.push((token.to_string(), next_token.to_string()));
                    tokens.next();
                },
                None => {
                    eprintln!("syntax error: unexpected end of file");
                }
            }
        } else {
            command_line.push(token.to_string());
        }
    }
}

fn process_redirection(redirections: &Vec<(String, String)>) -> HashMap<String, File> {
    let mut file_map: HashMap<String, File> = HashMap::new();
    for (operator, file) in redirections {
        match operator.as_str() {
            ">" | "1>" => {
                let file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file)
                    .unwrap();
                file_map.insert("1".to_string(), file);
            },
            "2>" => {
                let file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file)
                    .unwrap();
                file_map.insert("2".to_string(), file);
            },
            _ => {
                eprintln!("syntax error: unknown redirection operator");
            }
        }
    }
    file_map
}
