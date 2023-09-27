use rust_bert::resources::{LocalResource, ResourceProvider};
use rust_bert::t5::{T5Config, T5ForConditionalGeneration};
use rust_tokenizers::tokenizer::T5Tokenizer;
use std::path::PathBuf;
use tch::{nn, Device};

fn main() -> anyhow::Result<()> {
    let config_resource = LocalResource {
        local_path: PathBuf::from("tgs/crates/tgs_t5_finetunned/model/config.json"),
    };
    let sentence_piece_resource = LocalResource {
        local_path: PathBuf::from("tgs/crates/tgs_t5_finetunned/model/spiece.model"),
    };
    let weights_resource = LocalResource {
        local_path: PathBuf::from("tgs/crates/tgs_t5_finetunned/model/pytorch_model.bin"),
    };

    let device = Device::cuda_if_available();
    let mut vs = nn::VarStore::new(device);

    let tokenizer = T5Tokenizer::from_file(
        sentence_piece_resource.get_local_path()?.to_str().unwrap(),
        true,
    )?;
    let config = T5Config::from_file(config_resource.get_local_path()?);
    let t5_model = T5ForConditionalGeneration::new(&vs.root(), &config);

    vs.load(weights_resource.get_local_path()?)?;

    // Your code for inference here

    Ok(())
}
