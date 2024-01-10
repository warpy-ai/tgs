use std::fs;
use std::io::{self};
use tgs_handler;
use tgs_prompt::{terminal_prompt, MyPrompt};
use tgs_setup;
use tgs_shell::{
    history::FileBackedHistory,
    keybindings,
    line::_core::shell::set_working_dir,
    prelude::{cursor_buffer::CursorBuffer, styled_buf::StyledBuf, *},
};
use tgs_t5_finetunned;
use tokio::runtime;

fn main() {
    let path = std::env::var("PATH").unwrap();
    let startup_msg: HookFn<StartupCtx> =
        |_sh: &Shell,
         _sh_ctx: &mut Context,
         _sh_rt: &mut Runtime,
         _ctx: &StartupCtx|
         -> anyhow::Result<()> { Ok(tgs_welcome::display_welcome_message()) };
    let runtime = runtime::Runtime::new().unwrap();

    let config = tgs_setup::TgsSetup::new();
    match config.setup() {
        Ok(_) => println!("TGS setup complete."),
        Err(e) => eprintln!("Error setting up TGS: {}", e),
    }

    let prompt = MyPrompt;
    let menu = DefaultMenu::default();

    let readline = LineBuilder::default()
        .with_menu(menu)
        .with_prompt(prompt)
        .build()
        .expect("Could not construct readline");

    let mut hooks = Hooks::new();
    hooks.insert(startup_msg);

    let config_dir = dirs::home_dir().unwrap().as_path().join(".config/tgs");
    fs::create_dir_all(config_dir.clone());
    let history_file = config_dir.as_path().join("history");
    let history = FileBackedHistory::new(history_file).expect("Could not open history file");

    let shell = ShellBuilder::default()
        .with_hooks(hooks)
        .with_readline(readline)
        .with_history(history)
        .build()
        .expect("Could not construct shell");

    shell.run();
}

/*
#[cfg(test)]
mod tests {
    use duct::cmd;

    #[test]
    fn test_tgs_shell_echo() {
        // Simulate user input "echo Hello, world!" and "exit"
        println!("Should echo Hello, world!");
        let input = "echo Hello, world!\nexit\n";
        let output = cmd!("cargo", "run")
            .stdin_bytes(input)
            .read()
            .expect("Failed to run tgs_shell with input");

        // Print the actual output for debugging
        println!("Actual output: {}", output);

        // Check that the output contains the expected prompt and output
        assert!(output.contains("tgs> Hello, world!"));
    }

    #[test]
    fn test_tgs_shell_exit() {
        // Simulate user input "exit"
        println!("Should exit with status 200");
        let input = "exit\n";
        let output = cmd!("cargo", "run")
            .stdin_bytes(input)
            .read()
            .expect("Failed to run tgs_shell with input");

        // Check that the output contains the expected prompt
        assert!(output.contains("tgs> "));
    }
}
*/
