use std::io::{self, Write};

fn main() {
    loop {
        // 1. Print a prompt.
        print!("tgs> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.

        // 2. Read a line of input.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 3. Process the input (for now, we'll just trim whitespace).
        let processed_input = input.trim();

        // 4. Display the output or result (for now, just echoing back).
        println!("You entered: {}", processed_input);
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
