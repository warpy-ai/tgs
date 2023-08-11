fn main() {
    loop {
        print!("tgs> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // TODO: Process input
        println!("You entered: {}", input.trim());
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
