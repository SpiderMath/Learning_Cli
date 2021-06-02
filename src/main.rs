use std::env::args;

fn main() {
    let arguments: Vec<String> = args().collect();

    // arguments[0] = 'target/release/cli.exe'

    println!("{:?}", arguments)
}