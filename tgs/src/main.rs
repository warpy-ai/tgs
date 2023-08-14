use std::io::{self, Write};
use tgs_shell;

fn main() {
    let raw_vars: Vec<String> = std::env::args().collect::<Vec<String>>();

    loop {
        // 1. Print a prompt.
        print!("tgs> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.

        // 2. Read a line of input.
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // process::run_process(&raw_vars, &input);

        // 3. Process the input using Zsh.
        let (code, output, error) = tgs_shell::execute("zsh", &input);

        // 4. Display the output or error.
        if code == 0 {
            println!("{}", output);
        } else {
            eprintln!("Error: {}", error);
        }
    }
}
