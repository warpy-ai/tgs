use std::{
    io,
    path::{Path, PathBuf},
};

pub fn find_binary(command: &str, path: &str) -> Result<PathBuf, std::io::Error> {
    fn search(command: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in std::fs::read_dir(path)? {
            if let Ok(entry) = entry {
                if let Ok(met) = entry.metadata() {
                    if met.is_file() || met.is_symlink() {
                        if let Some(name) = entry.path().file_name() {
                            if name == command {
                                if met.is_symlink() {
                                    panic!("symlink not supported");
                                }
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(std::io::ErrorKind::NotFound.into())
    }

    if let Ok(mut dir) = std::env::current_dir() {
        if let Ok(()) = search(command, &dir) {
            dir.push(command);
            return Ok(dir);
        }
    }

    for entry in path.split(":") {
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(command, &path) {
            path.push(command);
            return Ok(path);
        }
    }
    Err(std::io::ErrorKind::NotFound.into())
}
