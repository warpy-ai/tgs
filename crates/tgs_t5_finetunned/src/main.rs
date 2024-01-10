mod from_py;
use from_py::execute;

fn main() {
    // TODO: Use lib function return_command and check returned value
    println!("Command: {}", "Prints sorted list of logged in users.");
    let result = execute("list all files in this directory");

    println!("Translation result: {:?}", result);
}
