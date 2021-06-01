use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    // arguments will always have the value of "target\\debug\\todo_cli.exe" at index 0
    if arguments.len() == 1 { return; }

    // If there is some command then send it
    let command = arguments[1].clone();

    println!("{}", command);
}
