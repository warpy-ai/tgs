import os
import torch
from transformers import T5ForConditionalGeneration, T5Tokenizer


def load_model(relative_model_path=None):
    # Get the directory where the script is located
    base_path = os.path.dirname(os.path.abspath(__file__))

    # The model directory is in the same parent directory as the script
    # Adjust this to the correct relative path
    if relative_model_path is None:
        relative_model_path = "crates/tgs_t5_finetunned/model"

    # Construct the absolute path to the model directory
    model_path = os.path.join(base_path, relative_model_path)

    # Load the model from the specified path
    model = T5ForConditionalGeneration.from_pretrained(model_path)

    # Load the model weights
    model.load_state_dict(torch.load(
        os.path.join(model_path, "pytorch_model.bin"),
        map_location=torch.device('cpu')))

    # Set the model to evaluation mode
    model.eval()

    # Load tokenizer
    tokenizer = T5Tokenizer.from_pretrained(model_path)

    return model, tokenizer


def generate_answer(input_text, relative_model_path=None, max_length=50):
    # Load the model and tokenizer
    model, tokenizer = load_model(relative_model_path)

    # Ensure the model is in evaluation mode
    model.eval()

    # Tokenize the input text
    input_ids = tokenizer.encode(input_text, return_tensors="pt")

    # Generate the output
    output_ids = model.generate(input_ids, max_length=max_length)[0]

    # Decode the output
    answer = tokenizer.decode(output_ids, skip_special_tokens=True)
    return answer
