/*
Citation:
```
@inproceedings{eval,
    title = "Nut nsh - A POSIX shell",
    author = "Seiya Nuta",
    year = "2023",
    url = "https://github.com/nuta/nsh/blob/main/src/eval.rs",
}
```
*/

use std::process::ExitStatus;

use tgs_utils::{run_external_command, JobManager, Output, Process, ProcessGroup, Stdin};

use crate::{ast, PosixError};

pub struct Os {
    _job_manager: JobManager,
    /// Exit status of last command executed.
    _last_exit_status: ExitStatus,
}

pub fn run_job(
    job_manager: &mut JobManager,
    procs: Vec<Box<dyn Process>>,
    pgid: Option<u32>,
    foreground: bool,
) -> Result<(), PosixError> {
    let proc_group = ProcessGroup {
        id: pgid,
        processes: procs,
        foreground,
    };

    let is_foreground = proc_group.foreground;
    let job_id = job_manager.create_job("", proc_group);

    if is_foreground {
        job_manager
            .put_job_in_foreground(Some(job_id), false)
            .map_err(|e| PosixError::Job(e))?;
    } else {
        job_manager
            .put_job_in_background(Some(job_id), false)
            .map_err(|e| PosixError::Job(e))?;
    }
    Ok(())
}

/// Returns group of processes and also the pgid if it has one
pub fn eval_command(
    job_manager: &mut JobManager,
    cmd: &ast::Command,
    stdin: Option<Stdin>,
    stdout: Option<Output>,
) -> Result<(Vec<Box<dyn Process>>, Option<u32>), PosixError> {
    match cmd {
        ast::Command::Simple { args, .. } => {
            // Attempt to execute the command, handling potential errors
            match execute_simple_command(args, stdin, stdout) {
                Ok(result) => Ok(result),
                Err(e) => {
                    // Handle the error, e.g., log it and continue
                    eprintln!("Error executing command: {:?}", e);
                    // Optionally return a "success" with no processes to avoid termination
                    // Adjust based on your error handling strategy
                    Ok((vec![], None))
                }
            }
        }
        ast::Command::Pipeline(a_cmd, b_cmd) => {
            let (mut a_procs, _a_pgid) =
                eval_command(job_manager, a_cmd, stdin, Some(Output::CreatePipe))?;
            let (b_procs, b_pgid) = eval_command(
                job_manager,
                b_cmd,
                a_procs.last_mut().unwrap().stdout(),
                stdout,
            )?;
            a_procs.extend(b_procs);
            Ok((a_procs, b_pgid))
        }
        ast::Command::AsyncList(a_cmd, b_cmd) => {
            // TODO double check stdin and stdout
            let (procs, pgid) = eval_command(job_manager, a_cmd, None, None)?;
            run_job(job_manager, procs, pgid, false)?;

            if let Some(b_cmd) = b_cmd {
                eval_command(job_manager, b_cmd, None, None)
            } else {
                Ok((vec![], None))
            }
        }
        ast::Command::None => {
            println!("Command not found");
            Ok((vec![], None))
        }
        _ => Err(PosixError::Eval(anyhow::Error::msg(
            "Command not yet implemented",
        ))),
    }
}

fn execute_simple_command(
    args: &[String], // Assuming args is a slice of String
    stdin: Option<Stdin>,
    stdout: Option<Output>,
) -> Result<(Vec<Box<dyn Process>>, Option<u32>), PosixError> {
    let program = args
        .first()
        .ok_or(PosixError::CommandNotFound("No command specified".into()))?;
    // Safely unwrap `stdin` and `stdout`, providing defaults if they're None
    let stdin_unwrapped = stdin.unwrap_or(Stdin::Inherit);
    let stdout_unwrapped = stdout.unwrap_or(Output::Inherit);

    // Call `run_external_command` with unwrapped values
    match run_external_command(
        program,
        &args[1..],
        stdin_unwrapped,
        stdout_unwrapped,
        Output::Inherit,
        None,
    ) {
        Ok((proc, pgid)) => Ok((vec![proc], pgid)),
        Err(e) => Err(PosixError::Eval(e.into())), // Convert the error as needed
    }
}
