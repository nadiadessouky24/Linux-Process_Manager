use std::io::{self, Write};

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Error: unable to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: unable to read input");
    input.trim().to_string()
}
