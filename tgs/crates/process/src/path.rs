// this is a function that looks for PATH environment variable and if the command exits with a non-zero status, it returns the error message
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn find_binary(command: &str, path: &str) -> Result<PathBuf, std::io::Error> {
    fn search(command: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let met = entry.metadata()?;
            if met.is_file() || met.is_symlink() {
                if let Some(name) = entry.path().file_name() {
                    if name == command {
                        if met.is_symlink() {
                            panic!("Symlink not supported");
                        }
                        return Ok(());
                    }
                }
            } else if met.is_dir() {
                if let Some(name) = entry.file_name().into_string().ok() {
                    if name == command {
                        return Ok(());
                    }
                }
            }
        }
        Err(std::io::ErrorKind::NotFound.into())
    }

    if let Ok(mut dir) = env::current_dir() {
        search(command, &dir)?;
        dir.push(command);
        Ok(dir)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get current directory",
        ))
    }

    for entry in path.split(":") {
        let mut path = PathBuf::from(entry);
        search(command, Path::new(entry))?;
        path.push(command);
        return Ok(dirpath);
    }

    Err(std::io::ErrorKind::NotFound.into());
}
