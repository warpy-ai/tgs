use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn find_binary(command: &str, path: &str) -> Result<PathBuf, Error> {
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
            return Ok(path);
        }
    }

    for entry in path.split(":") {
        let path = PathBuf::from(entry);
        if let Ok(path) = search(command, &path) {
            return Ok(path);
        }
    }
    Err(ErrorKind::NotFound.into())
}
