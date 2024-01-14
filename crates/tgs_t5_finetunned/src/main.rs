mod from_py;
use from_py::execute;

fn main() {
    // TODO: Use lib function return_command and check returned value
    let test_command = "list all mp3 files in the current";
    println!("Command: {}", test_command);
    let result = execute(test_command);

    println!("Translation result: {:?}", result);
}
