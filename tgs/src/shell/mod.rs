pub fn execute(shell: &str, cmd: &String) -> (i32, String, String) {
    let mut command = {
        let mut command = ::std::process::Command::new(shell);
        command.arg("-c").arg(cmd);
        command
    };

    match command.output() {
        Ok(output) => (
            output
                .status
                .code()
                .unwrap_or(if output.status.success() { 0 } else { 1 }),
            String::from_utf8_lossy(&output.stdout[..]).into_owned(),
            String::from_utf8_lossy(&output.stderr[..]).into_owned(),
        ),

        Err(e) => (126, String::new(), e.to_string()),
    }
}
