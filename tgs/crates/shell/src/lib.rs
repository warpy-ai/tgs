use std::io;
use std::process::ExitStatus;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub fn execute(bin: &str, args: &[&str]) -> Result<ExitStatus, io::Error> {
    // Handle built-in commands
    let command_name = std::path::Path::new(bin)
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary path"))?
        .to_str()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in binary path")
        })?;

    match command_name {
        "cd" => {
            let new_dir = args.get(0).map_or(".", |x| *x); // Default to the current directory if no argument is provided
            std::env::set_current_dir(new_dir)?;
            Ok(ExitStatus::from_raw(0)) // Return a successful exit status
        }
        "echo" => {
            println!("{}", args.join(" "));
            Ok(ExitStatus::from_raw(0))
        }
        _ => {
            // Execute the command
            let mut command = {
                let mut command = ::std::process::Command::new(bin);
                command.args(args);
                command
            };

            match command.output() {
                Ok(output) => {
                    let exit_code =
                        output
                            .status
                            .code()
                            .unwrap_or(if output.status.success() { 0 } else { 1 });
                    println!("{}", String::from_utf8_lossy(&output.stdout[..]));
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr[..]));
                    Ok(ExitStatus::from_raw(exit_code))
                }
                Err(e) => {
                    eprintln!("{}", e);
                    Ok(ExitStatus::from_raw(126))
                }
            }
        }
    }
}
