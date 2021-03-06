use std::{
    io::{self, Write},
    process::{exit, Command},
};

use crate::lexer::Lexer;

pub fn run() {
    println!(
        "Hello {}! This is the Monky programming language!",
        username()
    );

    let prompt = "monkey> ";
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim_end() == "exit" {
            exit(0);
        }

        let l = Lexer::new(input);
        l.for_each(|t| println!("{:?}", t));
    }
}

fn username() -> String {
    let whoami = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&whoami.stdout).trim().to_string()
}
