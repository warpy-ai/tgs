use dialoguer::{theme::ColorfulTheme, Select};
use pyo3::{prelude::*, types::PyModule};
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
            String::from("TODO")
        }
    } else {
        String::from("TODO")
    }
}

pub fn execute(input_text: &str) -> PyResult<String> {
    let loader = LoadingIndicator::new(custom::DARK_WHITE);
    pyo3::prepare_freethreaded_python();
    // Construct the absolute path to the Python script
    let mut script_path = env::current_exe()?;
    // Navigate up to the project root directory
    for _ in 0..3 {
        script_path.pop();
    }
    // Now script_path should point to the project root
    script_path.push("crates/tgs_t5_finetunned/inference_model.py"); // Navigate to the script

    loader.start(input_text);
    Python::with_gil(|py| {
        let code = fs::read_to_string(script_path)?;

        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();
        loader.stop();
        Ok(call_dialoger(result?))
    })
}
