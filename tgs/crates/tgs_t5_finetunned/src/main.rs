use tgs_t5_finetunned::return_command;

fn main() {
    // TODO: Use lib function return_command and check returned value
    println!("Command: {}", "Prints sorted list of logged in users.");
    let result = return_command("Prints sorted list of logged in users.");

    println!("Translation result: {:?}", result);
}
