use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {
    // Spawn Zsh as a subprocess
    let mut child = Command::new("zsh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Zsh subprocess");

    loop {
        // 1. Print a prompt.
        print!("tgs> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.

        // 2. Read a line of input.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 3. Send the input to Zsh for execution.
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();

        // 4. Capture the output from Zsh.
        let mut output = Vec::new();
        child
            .stdout
            .as_mut()
            .unwrap()
            .read_to_end(&mut output)
            .unwrap();
        let output_str = String::from_utf8_lossy(&output);

        // 5. Display the output.
        println!("{}", output_str);
    }
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("tgs"), "sgt");
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
    }
}
