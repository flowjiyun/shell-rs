use std::{collections::HashMap, fs::{self, File}};

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

#[derive(Debug)]
pub struct Command {
    _type: CommandType,
    _prog: String,
    _args: Vec<String>,
    _file_map: HashMap<String, File>,
}

impl Command {
    pub fn new(prog: String, args: Vec<String>, file_map: HashMap<String, File>) -> Self {
        // init command type
        let command_type = Self::_check_command_type(&prog);
        Self {
            _type: command_type,
            _prog: prog,
            _args: args,
            _file_map: file_map,
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

    pub fn execute(&mut self) {
        match self._type {
            CommandType::BuiltIn(builtin) => {
                match builtin  {
                    BuiltInImpl::Exit => {
                        bt_exit(self._args.clone());
                    },
                    BuiltInImpl::Echo => {
                        bt_echo(self._args.clone(), &mut self._file_map);

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
                Self::_execute_external(&self._prog, &self._args, &mut self._file_map);
            }
        }
    }

    fn _execute_external(prog: &str, args: &Vec<String>, file_map: &mut HashMap<String, File>) {
        let env_path = std::env::var("PATH").unwrap_or_default();
        let paths: Vec<&str> = env_path.split(":").collect();
        for path in paths {
            let full_path = format!("{}/{}", path, prog);
            if fs::metadata(&full_path).is_ok() {
                if let Some(stdout_file) = file_map.get_mut("1") {
                    if let Ok(mut command) = std::process::Command::new(full_path.as_str()) 
                        .args(args)
                        .stdout(stdout_file.try_clone().unwrap())
                        .spawn() {
                            let _ = command.wait();
                    }
                    return;
                } else {
                    if let Ok(mut command) = std::process::Command::new(full_path.as_str())
                        .args(args)
                        .spawn() {
                            let _ = command.wait();
                    }
                    return;
                }
            }
        }
        println!("{}: command not found", prog);
    }
}