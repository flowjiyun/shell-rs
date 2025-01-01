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

        let command_line = preprocess_input(input.trim());
        if command_line.is_empty() {
            continue;
        }
        let prog = command_line[0].trim();
        let args = command_line[1..].to_vec();

        let command = Command::new(prog.to_string(), args);
        command.execute();
    }
}

fn preprocess_input(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur_token = String::new();
    let mut is_in_quote = false;
    let mut is_in_escape = false;
    let mut quote_char = '\0';
    for c in input.chars() {
        match c {
            '\'' | '\"' => {
                if !is_in_quote {
                    if is_in_escape {
                        cur_token.push(c);
                        is_in_escape = false;
                    } else {
                        is_in_quote = true;
                        quote_char = c;
                    }
                } else {
                    if quote_char == c {
                        is_in_quote = false;
                    } else {
                        cur_token.push(c);
                    }
                }
            },
            '\\' => {
                if is_in_quote {
                    cur_token.push(c);
                } else {
                    if is_in_escape {
                        cur_token.push(c);
                        is_in_escape = false;
                    } else {
                        is_in_escape = true;
                    }
                }
            },
            ' ' => {
                if is_in_quote {
                    cur_token.push(c);
                } else {
                    if !cur_token.is_empty() {
                        if is_in_escape {
                            cur_token.push(c);
                            is_in_escape = false;
                        } else {
                            tokens.push(cur_token.clone());
                            cur_token.clear();
                        }
                    }
                }
            },
            _ => {
                cur_token.push(c);
            }
        }
    }
    if !cur_token.is_empty() {
        tokens.push(cur_token.clone());
    }
    tokens
}
