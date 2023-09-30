// Necessary imports
use anyhow::Result;
use rust_bert::resources::{LocalResource, ResourceProvider};
use rust_bert::t5::{T5Config, T5ForConditionalGeneration};
use rust_bert::Config;
use rust_tokenizers::tokenizer::{T5Tokenizer, Tokenizer, TruncationStrategy};
use std::path::{Path, PathBuf};
use tch::{nn, Device};

pub fn return_command(input_text: &str) -> Result<String> {
    // Define the paths for the model's configuration, tokenizer, and weights
    println!("1. Loading model...");
    let root_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    let config_path = root_path.join("model/config.json");
    let config_resource = LocalResource {
        local_path: PathBuf::from(config_path),
    };

    let spiece_path = root_path.join("model/spiece.model");
    let sentence_piece_resource = LocalResource {
        local_path: PathBuf::from(spiece_path),
    };

    let weights_path = root_path.join("model/pytorch_model.bin");
    let weights_resource = LocalResource {
        local_path: PathBuf::from(weights_path),
    };

    println!("2. Getting local paths...");
    let config_path = config_resource.get_local_path()?;
    let spiece_path = sentence_piece_resource.get_local_path()?;
    let weights_path = weights_resource.get_local_path()?;

    println!("3. Initializing device and var store...");
    // Check for available CUDA device
    let device = Device::cuda_if_available();
    let mut vs = nn::VarStore::new(device);

    println!("4. Loading model configuration and tokenizer...");
    let config = T5Config::from_file(config_path);
    let t5_model = T5ForConditionalGeneration::new(&vs.root(), &config);
    let tokenizer = T5Tokenizer::from_file(spiece_path.to_str().unwrap(), false)?;
    vs.load(weights_path)?;

    // Mock input for now
    println!("5. Tokenizing input...");
    let tokens = tokenizer.encode(input_text, None, 512, &TruncationStrategy::DoNotTruncate, 0);
    println!("Tokens: {:?}", tokens);

    // Convert TokenizedInput to Tensor
    let input_tensor: tch::Tensor = tokens.token_ids.as_slice().into();

    println!("Number of tokens: {}", tokens.token_ids.len());
    println!("6. Tokens: {}", input_tensor);

    let input_tensor = input_tensor.unsqueeze(0); // adds a batch dimension
    println!("Input tensor: {}", input_tensor);
    //   println!("Input tensor shape: {:?}", input_tensor.size::<i32>());

    // Inference using forward_t with all required arguments
    println!("7. Running inference...");

    // let decoder_input =
    //    tch::Tensor::of_slice(&[tokenizer.bos_token_id().unwrap(); 512]).unsqueeze(0);

    let decoded_tokens = tokenizer.decode(&tokens.token_ids, true, true);
    println!("Decoded tokens: {}", decoded_tokens);

    let model_output = t5_model.forward_t(
        Some(&input_tensor),
        None,                // attention_mask
        None,                // encoder_output
        Some(&input_tensor), // decoder_input
        None,                // position_ids
        None,                // encoder_attention_mask
        None,                // past_key_values
        None,                // use_cache
        false,               // train
    );

    // Extract the logits from the model output
    let output = model_output.decoder_output;

    println!("8. Converting tensor to Vec<i64>...");
    let output_ids = output.argmax(-1, false).view([-1]);
    let output_vec: Vec<i64> = output_ids.iter::<i64>().unwrap().collect();

    println!("output_vec: {:?}", output_vec);

    // Decode the predictions
    println!("9. Decoding predictions...");
    let decoded_output = tokenizer.decode(&output_vec[..], true, true);

    // Print the decoded output
    println!("10. Output: {}", decoded_output);

    Ok(decoded_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        // Call the main function
        let result = return_command("list all files in a directory");

        // Print the result for inspection
        println!("Translation result: {:?}", result);

        // Assert specific conditions on the result
        // For example, ensure the result is not empty

        Ok(())
    }
}
