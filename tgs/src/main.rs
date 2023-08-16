use std::io::{self, Write};
use tgs_handler;
use tgs_shell;
fn main() {
    let path = std::env::var("PATH").unwrap();
    loop {
        // 1. Print a prompt.
        print!("tgs> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.

        // 2. Read a line of input.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); // Trim whitespace

        // 3. Exit the shell if the user types "exit".
        if input == "exit" {
            break;
        }

        // 4. Execute the command using tgs_handler.
        let value: Vec<&str> = input.split(" ").collect();
        let bin = tgs_handler::find_binary(value[0], &path);

        // 5. Execute the command using tgs_shell
        match bin {
            Ok(bin) => {
                let bin_str = bin.to_str().unwrap(); // Convert PathBuf to &str
                match tgs_shell::execute(bin_str, &value[1..]) {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("Command exited with status: {}", exit_status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
