// build.rs
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    println!(
        "cargo:warning=CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let output_path = get_output_path();

    println!("cargo:warning=Calculated build path: {:?}", output_path);

    let input_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("crates/tgs_t5_finetunned/");

    if let Ok(context) = env::var("CUSTOM_BUILD_CONTEXT") {
        if context == "run" {
            // Special handling for `cargo run`
            println!("Handling for cargo run");
        }
    }

    // Define the destination paths
    let model_dest = input_path.join("model");

    let joinable_path = output_path.unwrap();
    let out_model_dest = joinable_path.join("model");
    let script_dest = input_path.join("inference_model.py");
    println!("Attempting to copy from: {}", script_dest.display());
    let out_dest = joinable_path.join("inference_model.py");

    // Copy model directory and inference_model.py to the target directory
    fs::copy(script_dest, out_dest).expect("Failed to copy inference_model.py");

    // Note: For directory copying, consider using a crate like `fs_extra`
    copy_dir_all(model_dest, out_model_dest).expect("Failed to copy model directory");
}

fn get_output_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let build_type = env::var("PROFILE").unwrap();

    let mut target_dir = None;
    let mut sub_path = manifest_dir_string.as_path();

    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&build_type) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
