use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use tgs_command::exit_handler;

type CommandHandler = fn() -> Result<(), Error>;
pub enum CommandType {
    Binary(PathBuf),
    Custom(String),
}

pub fn custom_commands() -> HashMap<String, CommandHandler> {
    let mut commands = HashMap::new();
    commands.insert("exit".to_string(), exit_handler as CommandHandler);
    commands
}

pub fn find_binary(command: &str, path: &str) -> Result<CommandType, Error> {
    let custom_cmds = custom_commands();
    if custom_cmds.contains_key(command) {
        return Ok(CommandType::Custom(command.to_string()));
    }

    fn search(command: &str, path: &Path) -> Result<PathBuf, Error> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let met = entry.metadata()?;
            if met.is_file() || met.is_symlink() {
                if let Some(name) = entry.path().file_name() {
                    if name == command {
                        if met.is_symlink() {
                            panic!("symlink not supported");
                        }
                        return Ok(entry.path());
                    }
                }
            }
        }
        Err(ErrorKind::NotFound.into())
    }

    if let Ok(dir) = std::env::current_dir() {
        if let Ok(path) = search(command, &dir) {
            return Ok(CommandType::Binary(path));
        }
    }

    for entry in path.split(":") {
        let path = PathBuf::from(entry);
        if let Ok(path) = search(command, &path) {
            return Ok(CommandType::Binary(path));
        }
    }
    Err(ErrorKind::NotFound.into())
}
