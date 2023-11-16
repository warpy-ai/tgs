use rust_bert::pipelines::question_answering::{Answer, QaInput, QuestionAnsweringModel};
use rust_bert::RustBertError;

pub fn nlp_module(line: String) -> Result<String, RustBertError> {
    let qa_model = QuestionAnsweringModel::new(Default::default())?;

    let question = line;

    // Assuming predict returns Vec<Vec<Answer>>
    let answers: Vec<Vec<Answer>> = qa_model.predict(
        &[QaInput {
            question: question.clone(),
            context: String::from("The capital of France is Paris."), // Provide a context
        }],
        1,
        32,
    );

    // Assuming the first answer of the first question is what you want
    if answers.is_empty() || answers[0].is_empty() {
        return Err(RustBertError::ValueError("No answers found".to_string()));
    }

    let answer = answers[0][0].answer.clone();

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::nlp_module; // Import the function you want to test

    #[test]
    fn it_works() {
        let question = "What is the capital of France?".to_string();
        let result = nlp_module(question).unwrap();

        assert_eq!(result, "Paris"); // Replace with your expected output
    }
}
