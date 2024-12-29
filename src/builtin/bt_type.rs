use crate::BUILTIN_SET;

pub fn bt_type(args: Vec<String>) {
    if let Some(builtin) = args.get(0) {
        match BUILTIN_SET.contains(builtin) {
            true => {
                println!("{} is a shell builtin", builtin);
            },
            false => {
                println!("{}: not found", builtin);
            }
        }
    }
}