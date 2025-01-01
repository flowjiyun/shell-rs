use std::fs;

use crate::{builtin::{bt_cd::bt_cd, bt_echo::bt_echo, bt_exit::bt_exit, bt_pwd::bt_pwd, bt_type::bt_type}, BUILTIN_SET};


#[derive(Debug, Clone, Copy)]
enum BuiltInImpl {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

#[derive(Debug, Clone, Copy)]
enum CommandType {
    BuiltIn(BuiltInImpl),
    External,
}

#[derive(Debug, Clone)]
pub struct Command {
    _type: CommandType,
    _prog: String,
    _args: Vec<String>,
}

impl Command {
    pub fn new(prog: String, args: Vec<String>) -> Self {
        // init command type
        let command_type = Self::_check_command_type(&prog);
        Self {
            _type: command_type,
            _prog: prog,
            _args: args,
        }

    }
    fn _check_command_type(prog: &str) -> CommandType {
        if let Some(builtin) = Self::_check_builtin(prog) {
           return CommandType::BuiltIn(builtin);
        } else {
           return CommandType::External;
        }
    }

    fn _check_builtin(prog: &str) -> Option<BuiltInImpl> {
        if BUILTIN_SET.contains(prog) {
            match prog {
                "exit" => Some(BuiltInImpl::Exit),
                "echo" => Some(BuiltInImpl::Echo),
                "type" => Some(BuiltInImpl::Type),
                "pwd" => Some(BuiltInImpl::Pwd),
                "cd" => Some(BuiltInImpl::Cd),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn execute(&self) {
        match self._type {
            CommandType::BuiltIn(builtin) => {
                match builtin  {
                    BuiltInImpl::Exit => {
                        bt_exit(self._args.clone());
                    },
                    BuiltInImpl::Echo => {
                        bt_echo(self._args.clone());

                    },
                    BuiltInImpl::Type => {
                        bt_type(self._args.clone());
                    },
                    BuiltInImpl::Pwd => {
                        bt_pwd();
                    },
                    BuiltInImpl::Cd => {
                        bt_cd(self._args.clone());
                    }
                }
            },
            CommandType::External => {
                Self::_execute_external(&self._prog, &self._args);
            }
        }
    }

    fn _execute_external(prog: &str, args: &Vec<String>) {
        let env_path = std::env::var("PATH").unwrap_or_default();
        let paths: Vec<&str> = env_path.split(":").collect();
        for path in paths {
            let full_path = format!("{}/{}", path, prog);
            if fs::metadata(&full_path).is_ok() {
                if let Ok(mut command) = std::process::Command::new(full_path.as_str()) 
                    .args(args)
                    .spawn() {
                        let _ = command.wait();
                    }
                return;
            }
        }
        println!("{}: command not found", prog);
    }
}