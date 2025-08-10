# markov-text

A CLI implementation of text generation using markov chains in Rust.

# Demo

<video src="https://hc-cdn.hel1.your-objectstorage.com/s/v3/f3658a867a57f23ca7cd042368f1bca63e32c6c3_screencast_20250810_155055.mp4" width="320" height="240" controls></video>

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