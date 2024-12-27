pub fn bt_echo(args: Vec<&str>) {
    let mut output = String::new();
    for arg in args {
        output.push_str(arg);
        output.push(' ');
    }
    println!("{}", output.trim());
}