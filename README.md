# TGS (Terminal Generative Shell)

![TGS Logo](https://www.warpy.io/_next/static/media/tgshell_icon.4fa45b6d.svg) <!-- If you have a logo, you can link it here -->

TGS is a conversational shell that combines the power of Rust with the intelligence of GPT-based natural language processing. Designed as a spinoff of the TerminalGPT project, TGS aims to redefine the way users interact with their terminals.

## Features

- **Conversational Interface**: Interact with your terminal in natural language. Instead of remembering complex commands, just tell TGS what you want to do.
- **Rust Integration**: Built with the robustness and efficiency of Rust, ensuring a fast and reliable terminal experience.
- **Intelligent Suggestions**: TGS provides smart suggestions and corrections, making your terminal experience smoother and more intuitive.
- **Safety Measures**: TGS has built-in safety checks to prevent accidental execution of potentially harmful commands.

## Installation

```bash
# Placeholder for installation commands
git clone <repository_link>
cd tgs
./install.sh
```

## Usage

Start TGS with the following command:

```bash
tgs
```

Once inside TGS, you can start conversing with your terminal:

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

## License

TGS is licensed under the [MIT License](path_to_license.md).

## Research references

- [Translating Natural Language to Bash Commands using Deep Neural Networks](https://web.stanford.edu/class/archive/cs/cs224n/cs224n.1224/reports/custom_116997097.pdf)
- [NL2CMD: An Updated Workflow for Natural Language to Bash Commands Translation](https://arxiv.org/pdf/2302.07845.pdf)
- [Dataset](https://github.com/magnumresearchgroup/magnum-nlc2cmd)
- [End-to-end NLP Pipelines in Rust](https://aclanthology.org/2020.nlposs-1.4.pdf)

---

Feel free to customize the README further to match the specific features, installation steps, and other details of your project. Remember to replace placeholders (like `<repository_link>`, `path_to_logo.png`, etc.) with actual links or paths as needed.

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

In case of running into `torch-sys` error refer to this [solution](https://github.com/LaurentMazare/tch-rs/issues/488#issuecomment-1664261286)
