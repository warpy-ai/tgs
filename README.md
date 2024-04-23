<p align="center">
  <img width="200" alt="TGS logo" src="https://github.com/warpy-ai/tgs/assets/11979969/1349328d-83e2-49d8-bebe-e61af6e9ae60"/>
</p>

<p align="center">
   <img width="80" alt="TGS" src="https://img.shields.io/github/actions/workflow/status/warpy-ai/tgs/pr.yml"/>
   <img width="100" alt="TGS" src="https://img.shields.io/github/contributors/warpy-ai/tgs"/>
</p>

# TGS (Terminal Generative Shell)

TGS is a conversational shell that combines Rust's power with the intelligence of GPT-based natural language processing. Designed as a spinoff of the TerminalGPT project, it aims to redefine the way users interact with their terminals.

## Features

- **Conversational Interface**: Interact with your terminal in natural language. Instead of remembering complex commands, tell TGS what you want to do.
- **Rust Integration**: It is built with the robustness and efficiency of Rust, ensuring a fast and reliable terminal experience.
- **Intelligent Suggestions**: TGS provides smart suggestions and corrections, making your terminal experience smoother and more intuitive.
- **Safety Measures**: TGS has built-in safety checks to prevent accidental execution of potentially harmful commands.

## Installation

You can install TGS using the following command. This script will automatically determine your OS, download the latest version of TGS, and install it:

```bash
curl -sSL https://raw.githubusercontent.com/warpy-ai/tgs/main/install.sh | bash
```

To install a specific version of TGS, append the version number to the command:

```bash
curl -sSL https://raw.githubusercontent.com/warpy-ai/tgs/main/install.sh | bash -s -- <version_number>
```

For example, to install version `v0.2.8`:

```bash
curl -sSL https://raw.githubusercontent.com/warpy-ai/tgs/main/install.sh | bash -s -- v0.2.8
```

## Usage

Start TGS with the following command:

```bash
tgs
```

Once inside TGS, you can start conversing with your terminal. Here are a few examples:

```
tgs> Create a new directory called "projects"
Directory "projects" created.

tgs> List all files in the current directory
...output...
```

## Testing

To run the unit tests for TGS:

```bash
cargo test
```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](path_to_contributing.md) for guidelines.

## Working with the project

This library relies on the [tch](https://github.com/LaurentMazare/tch-rs) crate for bindings to the C++ Libtorch API.
The `libtorch` library must be downloaded automatically or manually. The following references how to set up your environment
to use these bindings. Please refer to [tch](https://github.com/LaurentMazare/tch-rs) for detailed information and support.

Furthermore, this library relies on a cache folder for downloading pre-trained models.
The default cache folder is `~/.cache/.rustbert` but can be changed by setting the `RUSTBERT_CACHE` environment variable. Note that the language models used by this library are in the order of 100s of MBs to GBs.

### Manual installation (recommended)

1. Download `libtorch` from https://pytorch.org/get-started/locally/. This package requires `v2.1`: if this version is no longer available on the "get started" page,
   the file should be accessible by modifying the target link, for example `https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.1.1%2Bcu118.zip` for a Linux version with CUDA11. **NOTE:** When using `rust-bert` as a dependency from [crates.io](https://crates.io), please check the required `LIBTORCH` on the published package [readme](https://crates.io/crates/rust-bert) as it may differ from the version documented here (applying to the current repository version).
2. Extract the library to a location of your choice
3. Set the following environment variables

### Convert weights to .ot

```bash
python ./utils/convert_model.py model/pytorch_model.bin
```

##### Linux:

```bash
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

##### Windows

```powershell
$Env:LIBTORCH = "X:\path\to\libtorch"
$Env:Path += ";X:\path\to\libtorch\lib"
```

#### macOS + Homebrew

```bash
brew install pytorch jq
export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

### Automatic installation

Alternatively, you can let the `build` script automatically download the `libtorch` library. The `download-libtorch` feature flag needs to be enabled.
The CPU version of `libtorch` will be downloaded by default. To download a CUDA version, please set the environment variable `TORCH_CUDA_VERSION` to `cu118`.
Note that the libtorch library is large (several GBs for the CUDA-enabled version), and the first build may, therefore, take several minutes to complete.

### Verifying installation

Verify your installation (and linking with `libtorch`) by adding the `rust-bert` dependency to your `Cargo.toml` or by cloning the rust-bert source and running an example:

```bash
git clone git@github.com:guillaume-be/rust-bert.git
cd rust-bert
cargo run --example sentence_embeddings
```

## License

TGS is licensed under the [Apache License](https://github.com/warpy-ai/tgs?tab=readme-ov-file#Apache-2.0-1-ov-file).

## Research references

- [Translating Natural Language to Bash Commands using Deep Neural Networks](https://web.stanford.edu/class/archive/cs/cs224n/cs224n.1224/reports/custom_116997097.pdf)
- [NL2CMD: An Updated Workflow for Natural Language to Bash Commands Translation](https://arxiv.org/pdf/2302.07845.pdf)
- [Dataset](https://github.com/magnumresearchgroup/magnum-nlc2cmd)
- [End-to-end NLP Pipelines in Rust](https://aclanthology.org/2020.nlposs-1.4.pdf)

---

Feel free to customize the README further to match the specific features, installation steps, and other details of your project. Remember to replace placeholders (like `<repository_link>`, `path_to_logo.png`, etc.) with links or paths as needed.

## Citations

```
@inproceedings{becquin-2020-end,
    title = "End-to-end {NLP} Pipelines in Rust",
    author = "Becquin, Guillaume",
    booktitle = "Proceedings of Second Workshop for NLP Open Source Software (NLP-OSS)",
    year = "2020",
    publisher = "Association for Computational Linguistics",
    url = "https://www.aclweb.org/anthology/2020.nlposs-1.4",
    pages = "20--25",
}
```

## Troubleshooting

### For Mac.

In case of running into `torch-sys` error, refer to this [solution](https://github.com/LaurentMazare/tch-rs/issues/488#issuecomment-1664261286)

## Contributors

<a href="https://github.com/warpy-ai/tgs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=warpy-ai/tgs" />
</a>

## Acknowledgements

This project is a fork of the [shrs](https://github.com/MrPicklePinosaur/shrs) repository. The core functionality of the CLI architecture is derived from it. We want to express our gratitude to the original authors for their work. The original project is licensed under Apache-2.0/MIT.
