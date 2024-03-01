// build.rs
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    println!(
        "cargo:warning=CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let output_path = get_output_path();
    println!(
        "cargo:warning=Calculated build path: {}",
        output_path.to_str().unwrap()
    );

    let input_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("crates/tgs_t5_finetunned/");

    // Define the destination paths
    let model_dest = input_path.join("model");
    let out_model_dest = output_path.join("model");
    let script_dest = input_path.join("inference_model.py");
    let out_dest = output_path.join("inference_model.py");

    // Copy model directory and inference_model.py to the target directory
    fs::copy(script_dest, out_dest).expect("Failed to copy inference_model.py");

    // Note: For directory copying, consider using a crate like `fs_extra`
    copy_dir_all(model_dest, out_model_dest).expect("Failed to copy model directory");
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    // Check if a target triple is specified and adjust the path accordingly
    if let Ok(target_triple) = env::var("TARGET") {
        Path::new(&manifest_dir_string)
            .join("target")
            .join(target_triple) // Include the target triple in the path
            .join(build_type)
    } else {
        Path::new(&manifest_dir_string)
            .join("target")
            .join(build_type)
    }
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
