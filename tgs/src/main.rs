use std::io::{self, Write};
use tgs_bridge;
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

        match tgs_bridge::interpret_command(input) {
            Ok(interpreted_command) => {
                // Use interpreted_command for further processing
                println!("Interpreted Command: {}", interpreted_command);
            }
            Err(e) => {
                eprintln!("NLP Interpretation Error: {}", e);
                continue; // Skip the rest of the loop iteration
            }
        }

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

#[cfg(test)]
mod tests {
    use duct::cmd;

    #[test]
    fn test_tgs_shell_echo() {
        // Simulate user input "echo Hello, world!" and "exit"
        let input = "echo Hello, world!\nexit\n";
        let output = cmd!("cargo", "run")
            .stdin_bytes(input)
            .read()
            .expect("Failed to run tgs_shell with input");

        // Print the actual output for debugging
        println!("Actual output: {}", output);

        // Check that the output contains the expected prompt and output
        assert!(output.contains("tgs> Hello, world!"));
    }

    #[test]
    fn test_tgs_shell_exit() {
        // Simulate user input "exit"
        let input = "exit\n";
        let output = cmd!("cargo", "run")
            .stdin_bytes(input)
            .read()
            .expect("Failed to run tgs_shell with input");

        // Check that the output contains the expected prompt
        assert!(output.contains("tgs> "));
    }
}
