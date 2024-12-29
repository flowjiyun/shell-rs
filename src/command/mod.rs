use crate::{builtin::{bt_echo::bt_echo, bt_exit::bt_exit, bt_type::bt_type}, BUILTIN_SET};


#[derive(Debug, Clone, Copy)]
enum BuiltInImpl {
    Exit,
    Echo,
    Type,
}

#[derive(Debug, Clone, Copy)]
enum CommandType {
    BuiltIn(BuiltInImpl),
    Unknown,
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
           return CommandType::Unknown;
        }
    }

    fn _check_builtin(prog: &str) -> Option<BuiltInImpl> {
        if BUILTIN_SET.contains(prog) {
            match prog {
                "exit" => Some(BuiltInImpl::Exit),
                "echo" => Some(BuiltInImpl::Echo),
                "type" => Some(BuiltInImpl::Type),
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
                }
            },
            CommandType::Unknown => {
                println!("{}: command not found", self._prog);
            }
        }
    }
}