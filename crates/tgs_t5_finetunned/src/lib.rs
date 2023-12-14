use tch::{nn, no_grad, Device, Kind, Tensor};
// Assume a crate or module that can handle T5 models
use rust_bert::t5::{T5Config, T5ForConditionalGeneration};
use rust_bert::Config;
use rust_tokenizers::tokenizer::{T5Tokenizer, Tokenizer, TruncationStrategy};
use std::path::Path;

pub fn return_command(input_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let root_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    let device = Device::Cpu;
    // Load the model - adapt this part based on your Rust T5 handling library
    let config_path = root_path.join("model/config.json");
    let config = T5Config::from_file(config_path);
    let p = nn::VarStore::new(device);
    let spiece_path = root_path.join("model/spiece.model");
    let model = T5ForConditionalGeneration::new(&p.root() / "t5", &config);

    // Load the tokenizer
    let tokenizer = T5Tokenizer::from_file(spiece_path, false)?;

    // Test the model
    let answer = generate_answer(&model, &tokenizer, input_text, 50);
    Ok(answer)
}

fn generate_answer(
    model: &T5ForConditionalGeneration,
    tokenizer: &T5Tokenizer,
    input_text: &str,
    max_length: usize,
) -> String {
    let device = Device::Cpu;

    println!("Input text: {:?}", input_text);

    // Tokenize the input text
    let tokenized_input = tokenizer.encode(
        input_text,
        None,
        max_length,
        &TruncationStrategy::DoNotTruncate,
        max_length,
    );

    println!("Tokenized input IDs: {:?}", tokenized_input.token_ids);

    // Convert tokenized input to tensor
    let input_ids: Tensor = tokenized_input.token_ids.as_slice().into();

    // Create attention mask based on the length of the input
    let attention_mask = Tensor::ones(
        &[1, input_ids.size1().unwrap() as i64],
        (Kind::Int64, device),
    );

    // Placeholder for decoder input (adjust as necessary)
    let decoder_input_length = input_ids.size1().unwrap() as i64;
    let decoder_input_ids = Tensor::zeros(&[1, decoder_input_length], (Kind::Int64, device));

    let reshaped_input_tensor = input_ids.unsqueeze(0);
    println!("Tensor shape: {:?}", input_ids.size());
    println!("Input Tensor shape: {:?}", reshaped_input_tensor.size());
    println!("Attention Mask shape: {:?}", attention_mask.size());
    println!("Decoder Input IDs shape: {:?}", decoder_input_ids.size());

    // Call forward_t
    let model_output = no_grad(|| {
        model.forward_t(
            Some(&reshaped_input_tensor),
            Some(&attention_mask),
            None,
            Some(&decoder_input_ids),
            Some(&attention_mask),
            None,
            None,
            None,
            false,
        )
    });

    println!(
        "Model Output shape: {:?}",
        model_output.decoder_output.size()
    );

    // Assuming model_output.decoder_output is the tensor containing generated ids
    let output_tensor: Tensor = model_output.decoder_output;

    let output_prob = output_tensor.softmax(-1, tch::Kind::Float);
    let output_ids = output_prob.argmax(-1, true);

    println!("Output probabilities {:?}", output_prob);

    // Flatten the tensor to a 1D vector of token IDs
    let output_vec: Vec<i64> = output_ids.view([-1]).iter::<i64>().unwrap().collect();

    println!("Output id {:?}", output_ids);
    // Decode the output
    let decoded_output = tokenizer.decode(&output_vec[..], true, false);

    println!("Decoded output: {}", decoded_output);

    decoded_output
}
