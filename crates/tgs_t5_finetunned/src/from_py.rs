use pyo3::{prelude::*, types::PyModule};
use std::{env, fs};

pub fn execute(input_text: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    // Construct the absolute path to the Python script
    let mut script_path = env::current_exe()?;
    // Navigate up to the project root directory
    for _ in 0..3 {
        script_path.pop();
    }
    // Now script_path should point to the project root
    script_path.push("crates/tgs_t5_finetunned/inference_model.py"); // Navigate to the script

    println!("Generating script for: {:?}", input_text);
    Python::with_gil(|py| {
        let code = fs::read_to_string(script_path)?;

        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();

        result
    })
}
