use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};
use std::fs;

pub fn execute(input_text: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let code = fs::read_to_string("inference_model.py")?;
        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();

        println!("Answer: {:?}", result);
        result
    })
}
