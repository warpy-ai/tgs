use rust_bert::pipelines::text_generation::{
    GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources, ModelType,
    TextGenerationConfig,
};
use rust_bert::resources::RemoteResource;
use rust_bert::RustBertError;

pub fn nlp_module(line: String) -> Result<String, RustBertError> {
    let model_resource =
        Box::new(RemoteResource::from_pretrained(GptNeoModelResources::GPT_NEO_2_7B).unwrap());

    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));

    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));

    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 4,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    let model = TextGenerationConfig::new(generate_config);

    // Assuming model.generate returns a Result<String, RustBertError>
    let output = model.generate(&question)?;

    Ok(output[0].clone())
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
