pub mod util;

fn read() -> String {
    let mut rl = rustyline::Editor::<()>::new();

    if rl.load_history(".repl-history").is_err() {
        eprintln!("No previous history.");
    }

    let readline = rl.readline("user> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(&line);
            rl.save_history(".repl-history").unwrap();

            return format!("{}", line);
        }
        Err(_) => return "Type 'exit' to finish".to_string(),
    }
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
