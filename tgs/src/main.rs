fn main() {
    loop {
        print!("tgs> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // TODO: Process input
        println!("You entered: {}", input.trim());
    }
}
