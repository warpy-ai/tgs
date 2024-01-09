import torch
from transformers import T5ForConditionalGeneration
from transformers import T5Tokenizer


# Replace "t5-small" with the appropriate model size you fine-tuned
model = T5ForConditionalGeneration.from_pretrained("model")


model.load_state_dict(torch.load(
    "model/pytorch_model.bin", map_location=torch.device('cpu')))
model.eval()  # Set the model to evaluation mode


# Load tokenizer
tokenizer = T5Tokenizer.from_pretrained("model")


def generate_answer(input_text, max_length=50):
    model.eval()  # Ensure the model is in evaluation mode

    # Tokenize the input text
    input_ids = tokenizer.encode(input_text, return_tensors="pt")

    # Generate the output
    output_ids = model.generate(input_ids, max_length=max_length)[0]

    # Decode the output
    answer = tokenizer.decode(output_ids, skip_special_tokens=True)
    return answer
