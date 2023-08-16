use std::io;
use std::process::ExitStatus;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub fn execute(bin: &str, args: &[&str]) -> Result<ExitStatus, io::Error> {
    // Handle built-in commands
    match bin {
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
