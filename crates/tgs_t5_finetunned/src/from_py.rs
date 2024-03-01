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
            //TODO allow user to edit the command
            //TODO send edited command to server for model improvement
            String::from("TODO")
        }
    } else {
        String::from("TODO")
    }
}

pub fn execute(input_text: &str) -> PyResult<String> {
    let loader = LoadingIndicator::new(custom::DARK_WHITE);
    pyo3::prepare_freethreaded_python();
    let mut executable_path = env::current_exe()?;

    executable_path.pop(); // Remove the executable name from the path
    executable_path.push("inference_model.py"); // Directly point to `inference_model.py` at the root

    println!("Path: {:?}", executable_path);
    loader.start(input_text);
    Python::with_gil(|py| {
        let code = fs::read_to_string(&executable_path)?;

        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        println!("Module: {:?}", module);

        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();
        loader.stop();
        Ok(call_dialoger(result?))
    })
}
