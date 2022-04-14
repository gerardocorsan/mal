pub mod util;

use std::io::{stdin, stdout, Write};

fn read() -> String {
    let mut input = String::new();

    print!("user> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    return format!("{}", input.trim());
}

fn eval(input: &str) -> String {
    return String::from(input);
}

fn print(input: &str) -> &str {
    println!("{}", input);
    return input;
}

fn rep() -> String {
    let input = read();
    let output = eval(&input);
    print(&output);
    return output;
}

fn main() {
    let mut input = rep();
    while &input != "exit" {
        input = rep();
    }
}
