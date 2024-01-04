use directories::BaseDirs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub struct TgsSetup {
    config_path: String,
}

impl TgsSetup {
    pub fn new() -> Self {
        let base_dirs = BaseDirs::new().unwrap();
        let config_path = base_dirs
            .home_dir()
            .join(".tgsc")
            .to_str()
            .unwrap()
            .to_owned();
        TgsSetup { config_path }
    }

    pub fn setup(&self) -> io::Result<()> {
        if !Path::new(&self.config_path).exists() {
            self.create_tgsc()?;
        }
        Ok(())
    }

    fn create_tgsc(&self) -> io::Result<()> {
        let mut file = File::create(&self.config_path)?;
        writeln!(file, "# TGS Configuration")?;
        Ok(())
    }

    pub fn read_tgsc(&self) -> io::Result<String> {
        fs::read_to_string(&self.config_path)
    }

    pub fn update_tgsc_token(&self, token: &str) -> io::Result<()> {
        let mut tgsc_content = self.read_tgsc()?;
        tgsc_content.push_str(&format!("\nexport TGS_TOKEN={}", token));
        fs::write(&self.config_path, tgsc_content)
    }
}
