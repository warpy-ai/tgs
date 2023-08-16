use std::io;
use std::process::{Command, ExitStatus};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub fn execute(bin: &str, args: &[&str]) -> Result<ExitStatus, io::Error> {
    // Handle built-in commands
    match bin {
        "cd" => {
            let new_dir = args.get(0).map_or("/", |x| *x);
            std::env::set_current_dir(new_dir)?;
            Ok(ExitStatus::from_raw(0)) // Return a successful exit status
        }
        _ => {
            // Execute the command
            let mut command = Command::new(bin);
            command.args(args);
            let mut child = command.spawn()?;
            child.wait()
        }
    }
}
