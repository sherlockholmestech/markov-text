# markov-text

A CLI implementation of text generation using markov chains in Rust.

# Usage
To run the program, use the following command:

```bash
./markov-text <input_text_file> <state_size>
```
Where:
- `<input_text_file>` is the path to the text file you want to use as input.
- `<state_size>` is the number of words to consider as the state for the Markov chain.

# Example
```bash
./markov-text sherlock.txt 2
```