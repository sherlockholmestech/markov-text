use std::{collections::HashMap, fs};
use clap::{Parser, Subcommand};
use rand::seq::IndexedRandom;

/// A CLI program to quickly generate text using Markov chains, based on some input text.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a Markov chain model from the input text.
    Generate {
        /// The input text file to read from.
        #[arg(short, long)]
        input: String,

        /// The output file to write the generated model to.
        output: String,
        
        /// The size of the state to use for the Markov chain.
        #[arg(default_value_t = 2)]
        state_size: usize,
    },
    /// Generate text based on the Markov chain model.
    Model {
        /// The input file containing the Markov chain model.
        input: String,

        /// The number of words to generate.
        #[arg(default_value_t = 100)]
        max_words: usize,

        /// The size of the state to use for text generation.
        #[arg(default_value_t = 2)]
        state_size: usize,
    },
    /// Gnerate text from input text.
    Text {
        /// The input text file to read from.
        input: String,
        /// The number of words to generate.
        #[arg(default_value_t = 100)]
        max_words: usize,
        /// The size of the state to use for text generation.
        #[arg(default_value_t = 2)]
        state_size: usize,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { input, output, state_size } => {
            let input_text = fs::read_to_string(input).unwrap();
            let model = generate_markov_chain(&input_text, *state_size);
            println!("Markov model generated with {} states.", model.len());
            let output_model = serde_json::to_string(&model).unwrap();
            fs::write(output, output_model).expect("Unable to write model to file");
            println!("Model written to {}", output);
        },
        Commands::Model { input, max_words, state_size } => {
            let model_data = fs::read_to_string(input).unwrap();
            let model: HashMap<String, Vec<String>> = serde_json::from_str(&model_data).unwrap();
            println!("Markov model loaded with {} states.", model.len());
            let generated_text = generate_text(&model, *state_size, *max_words);
            println!("Here is the generated text:\n\n{}", generated_text);
        },
        Commands::Text { input, max_words, state_size } => {
            let input_text = fs::read_to_string(input).unwrap();
            let model = generate_markov_chain(&input_text, *state_size);
            println!("Markov model generated with {} states.", model.len());
            let generated_text = generate_text(&model, *state_size, *max_words);
            println!("Here is the generated text:\n\n{}", generated_text);
        },
    }

    // let input = fs::read_to_string("sherlock.txt").unwrap();
    // let model = generate_markov_chain(&input, 2);
    // println!("Markov model generated with {} states.", model.len());
    // let generated_text = generate_text(&model, 2, 300);
    // println!("Here is the generated text:\n\n{}", generated_text);
}

fn generate_markov_chain(input: &str, state_size: usize) -> HashMap<String, Vec<String>> {
    let mut input_vec: Vec<String> = Vec::new();
    let mut model: HashMap<String, Vec<String>> = HashMap::new();

    // Collect Words
    for line in input.lines() {
        for word in line.split(" ") {
            input_vec.push(word.to_string());
        }
    }
    println!("Collected {} words from input.", input_vec.len());

    // Cook up the Markov Chain with given state_size
    for i in state_size..(input_vec.len()) {
        let current_word = &input_vec[i];
        let previous_words = input_vec.get(i-state_size..i).unwrap().join(" ");
        if model.contains_key(&previous_words) {
            model.get_mut(&previous_words).unwrap().push(current_word.to_string());
        } else {
            model.insert(previous_words.clone(), vec![current_word.to_string()]);
        }
        println!("State: '{}', Next: '{}'", previous_words, current_word);
    }

    println!("Markov chain construction complete. States: {}", model.len());
    return model;
}

fn generate_text(model: &HashMap<String, Vec<String>>, state_size: usize, max_words: usize) -> String {
    let mut output_vec: Vec<String> = Vec::new();

    // Generate the text based off the markov model we generated

    // Get a valid starting character with a capital letter.
    let starter = get_text_starter(model, state_size);
    println!("Starter Chosen: {:#?}", starter);
    let starter_vec: Vec<&str> = starter.split(" ").collect();
    for i in starter_vec.iter() {
        output_vec.push(i.to_string());
    }

    // Get next state based on starter word
    for i in state_size..max_words {
        let previous_words = output_vec.get(i-state_size..i).unwrap().join(" ");
        println!("Current state: '{}'", previous_words);
        let next_word = match model.get(&previous_words) {
            Some(words) => {
                let chosen = words.choose(&mut rand::rng()).unwrap();
                println!("Next word chosen: '{}'", chosen);
                chosen
            }
            None => {
                println!("No next word found for state '{}', stopping generation.", previous_words);
                break;
            }
        };
        output_vec.push(next_word.to_string());
    }

    let output = output_vec.join(" ");
    return output;
}

fn get_text_starter(model: &HashMap<String, Vec<String>>, state_size: usize) -> String {
    let mut starters_all: Vec<String> = Vec::new();
    let mut starters_valid: Vec<String> = Vec::new();

    // Checks for capital letters in the start of the state
    for (key, value) in model.iter() {
        let capital_letters: [&str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
        for i in capital_letters.iter() {
            if key[0..1].contains(*i) {
                starters_all.push(key.to_string());
                break;
            }
        }

        // Check 2nd letter is not capital, to prevent proper nouns
        for (e, starter) in starters_all.iter().enumerate() {
            for i in capital_letters.iter() {
                if !starter.split(" ").collect::<Vec<&str>>().get(state_size-1).unwrap().contains(*i) {
                    starters_valid.push(starter.to_string());
                    break;
                }
            }
        }
    }
    // Randomly pick 1 to be the starter
    let mut random_num = rand::rng();
    let starter = starters_valid.choose(&mut random_num).unwrap().to_string();
    println!("Starter selected: '{}'", starter);
    return starter;
}