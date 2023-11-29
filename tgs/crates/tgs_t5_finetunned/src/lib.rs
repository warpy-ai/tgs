use anyhow::Result;
use rust_bert::resources::{LocalResource, ResourceProvider};
use rust_bert::t5::{T5Config, T5ForConditionalGeneration};
use rust_bert::Config;
use rust_tokenizers::tokenizer::{T5Tokenizer, Tokenizer, TruncationStrategy};
use std::path::Path;
use std::path::PathBuf;
use tch::{nn, no_grad, Device, Tensor};

fn load_model() -> Result<(T5ForConditionalGeneration, T5Tokenizer, nn::VarStore)> {
    let root_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    let config_path = root_path.join("model/config.json");
    let config_resource = LocalResource {
        local_path: PathBuf::from(config_path),
    };

    let spiece_path = root_path.join("model/spiece.model");
    let sentence_piece_resource = LocalResource {
        local_path: PathBuf::from(spiece_path),
    };

    let weights_path = root_path.join("model/rust_model.ot");
    let weights_resource = LocalResource {
        local_path: PathBuf::from(weights_path),
    };

    let config_path = config_resource.get_local_path()?;
    let spiece_path = sentence_piece_resource.get_local_path()?;
    let weights_path = weights_resource.get_local_path()?;

    let device = Device::cuda_if_available();
    let mut vs = nn::VarStore::new(device);

    let config = T5Config::from_file(config_path);
    let model = T5ForConditionalGeneration::new(&vs.root(), &config);
    let tokenizer = T5Tokenizer::from_file(spiece_path.to_str().unwrap(), false)?;
    vs.load(weights_path)?;

    Ok((model, tokenizer, vs))``
}

fn tokenize_input(tokenizer: &T5Tokenizer, input_text: &str) -> Tensor {
    let tokens = tokenizer.encode(input_text, None, 512, &TruncationStrategy::DoNotTruncate, 0);
    let input_tensor = Tensor::from(tokens.token_ids.as_slice());
    input_tensor.to_kind(tch::Kind::Int64).unsqueeze(0)
}

fn run_inference(model: &T5ForConditionalGeneration, input_tensor: &Tensor) -> Tensor {
    let model_output = no_grad(|| {
        model.forward_t(
            Some(input_tensor), // input ids
            None,               // attention mask (optional)
            None,               // encoder output (optional)
            None,               // decoder input ids (optional)
            None,               // decoder attention mask (optional)
            None,               // input embeds (optional)
            None,               // decoder input embeds (optional)
            None,               // old layer states (optional)
            false,              // train mode (false for inference)
        )
    });
    model_output.decoder_output
}

fn decode_output(tokenizer: &T5Tokenizer, output_tensor: &Tensor) -> String {
    let output_ids = output_tensor.argmax(-1, false).view([-1]);
    let output_vec: Vec<i64> = output_ids.iter::<i64>().unwrap().collect();
    tokenizer.decode(&output_vec[..], true, true)
}

pub fn return_command(input_text: &str) -> Result<String> {
    let (model, tokenizer, _var_store) = load_model()?;
    let input_tensor = tokenize_input(&tokenizer, input_text);
    println!("Input tensor shape: {:?}", input_tensor.size());
    println!("Input tensor content: {:?}", input_tensor);

    let output_tensor = run_inference(&model, &input_tensor);
    let decoded_output = decode_output(&tokenizer, &output_tensor);

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
