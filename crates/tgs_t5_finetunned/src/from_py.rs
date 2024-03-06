use dialoguer::{theme::ColorfulTheme, Select};
use pyo3::{prelude::*, types::PyModule};
use std::path::PathBuf;
use std::{env, fs};
use tgs_colors::custom;
use tgs_loader::LoadingIndicator;

fn call_dialoger(result: String) -> String {
    let multiselected = &[
        "\u{1f680} Execute command",
        "\u{270f} Edit and execute command",
        "\u{2718} Cancel",
    ];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt(result.clone())
        .items(&multiselected[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        if selection == 0 {
            result
        } else {
            //TODO allow user to edit the command
            //TODO send edited command to server for model improvement
            String::from("TODO")
        }
    } else {
        String::from("TODO")
    }
}

fn get_environment() -> String {
    std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
}

fn find_inference_model() -> Result<PathBuf, String> {
    let environment = get_environment();
    let mut path = PathBuf::new();

    match environment.as_str() {
        "production" => {
            println!("Running in production mode");
            // In production, the script might be located in a specific directory
            path.push("/path/to/production/location/inference_model.py");
        }
        "testing" => {
            println!("Running in testing mode");
            // For testing, the script might be elsewhere or named differently
            path.push("/path/to/testing/location/inference_model_test.py");
        }
        _ => {
            println!("Running in development mode");
            // For development and default, assume it's next to the executable or in a known dev path
            let mut exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
            exe_path.pop();
            exe_path.push("inference_model.py");
            if exe_path.exists() {
                return Ok(exe_path);
            }

            // Alternatively, look for the script relative to the manifest dir (useful in development)
            let dev_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
                .join("inference_model.py");
            if dev_path.exists() {
                return Ok(dev_path);
            }
        }
    }

    if path.exists() {
        Ok(path)
    } else {
        Err("Failed to find inference_model.py in any known location.".to_string())
    }
}

pub fn execute(input_text: &str) -> PyResult<String> {
    let loader = LoadingIndicator::new(custom::DARK_WHITE);
    pyo3::prepare_freethreaded_python();
    let executable_path = find_inference_model().expect("Failed to find inference_model.py");

    loader.start(input_text);

    Python::with_gil(|py| {
        let code = fs::read_to_string(&executable_path)?;

        println!(
            "Loading inference_model.py... {}",
            executable_path.display()
        );
        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        println!("Executing inference_model.py... {}", module);
        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();
        loader.stop();
        Ok(call_dialoger(result?))
    })
}
