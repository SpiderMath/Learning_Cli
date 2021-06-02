use std::env::args;

fn help() {
    println!("Displaying help menu...");
}

fn main() {
    let mut arguments: Vec<String> = args().collect();

    // arguments[0] = 'target/release/cli.exe'
    if arguments.len() == 1 {
        return help()
    }

    // Removing the unnecessary 'target/release/cli.exe', it ain't required
    arguments.remove(0);

    // Getting the command name
    let cmd_name: String= arguments[0].clone();

    // Then removing that too
    arguments.remove(0);

    println!("{}", cmd_name);
}