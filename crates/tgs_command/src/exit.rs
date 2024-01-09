use std;
use std::io::Error;

pub fn exit_handler() -> Result<(), Error> {
    // Implementation for the exit command
    println!("Exiting...");
    std::process::exit(0);
}
