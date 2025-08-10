# markov-text

A CLI implementation of text generation using markov chains in Rust.

# Demo

https://github.com/user-attachments/assets/b01be891-c839-4c47-80ed-b67b281e3815

# Usage
## Generation from a Text File
To run the program, use the following command:

```bash
./markov-text text <input_text_file> <max_words> <state_size>
```
Where:
- `<input_text_file>` is the path to the text file you want to use as input. An example file is `examples/sherlock.txt`, provided in the repository.
- `<state_size>` is the number of words to consider as the state for the Markov chain.
- `<max_words>` is the maximum number of words to generate. For example, `300` will generate 300 words.

## Generation from a markov model file
To generate text from a pre-existing Markov model, use the following command:
```bash
./markov-text model <model_file> <max_words> <state_size>
```
Where:
- `<model_file>` is the path to the Markov model file (in JSON format). An example file is `examples/sherlock.json`, provided in the repository.
- `<state_size>` is the number of words to consider as the state for the Markov chain. All example models use a state size of 2.

## Generate a Markov model from a text file
To generate a Markov model from a text file, use the following command:
```bash
./markov-text generate -input <input_text_file> <model_file> <state_size> 
```
Where:
- `<input_text_file>` is the path to the text file you want to use as input. An example file is `examples/sherlock.txt`, provided in the repository.
- `<model_file>` is the path where you want to save the generated Markov model (in JSON format).
- `<state_size>` is the number of words to consider as the state for the Markov chain.
