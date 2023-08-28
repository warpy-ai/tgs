use pyo3::prelude::*;

#[pyfunction]
pub fn interpret_command(command: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        // Include the Python code from example.py
        let py_code = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/modules/example.py"
        ));

        // Create a new Python module
        let nlp_module = PyModule::new(py, "nlp_module")?;

        // Execute the Python code in the context of the new module
        py.run(py_code, Some(nlp_module.dict()), None)?;

        // Get the Python function
        let py_function = nlp_module.getattr("interpret_command")?;

        // Prepare arguments for Python function call
        let args = (command,);

        // Call Python function and extract result
        let interpreted_command: String = py_function.call1(args)?.extract()?;

        Ok(interpreted_command)
    })
}
