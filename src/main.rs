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

    let result = match &cli.command {
        Commands::Generate { input, output, state_size } => {
            generate_command(input, output, *state_size)
        },
        Commands::Model { input, max_words, state_size } => {
            model_command(input, *max_words, *state_size)
        },
        Commands::Text { input, max_words, state_size } => {
            text_command(input, *max_words, *state_size)
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn generate_command(input: &str, output: &str, state_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let input_text = fs::read_to_string(input)
        .map_err(|e| format!("Failed to read input file '{}': {}", input, e))?;
    
    if input_text.trim().is_empty() {
        return Err("Input file is empty".into());
    }
    
    let model = generate_markov_chain(&input_text, state_size)?;
    println!("Markov model generated with {} states.", model.len());
    
    let output_model = serde_json::to_string(&model)
        .map_err(|e| format!("Failed to serialize model: {}", e))?;
    
    fs::write(output, output_model)
        .map_err(|e| format!("Failed to write model to file '{}': {}", output, e))?;
    
    println!("Model written to {}", output);
    Ok(())
}

fn model_command(input: &str, max_words: usize, state_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let model_data = fs::read_to_string(input)
        .map_err(|e| format!("Failed to read model file '{}': {}", input, e))?;
    
    let model: HashMap<String, Vec<String>> = serde_json::from_str(&model_data)
        .map_err(|e| format!("Failed to parse model file '{}': {}", input, e))?;
    
    if model.is_empty() {
        return Err("Model is empty".into());
    }
    
    println!("Markov model loaded with {} states.", model.len());
    let generated_text = generate_text(&model, state_size, max_words)?;
    println!("Here is the generated text:\n\n{}", generated_text);
    Ok(())
}

fn text_command(input: &str, max_words: usize, state_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let input_text = fs::read_to_string(input)
        .map_err(|e| format!("Failed to read input file '{}': {}", input, e))?;
    
    if input_text.trim().is_empty() {
        return Err("Input file is empty".into());
    }
    
    let model = generate_markov_chain(&input_text, state_size)?;
    println!("Markov model generated with {} states.", model.len());
    let generated_text = generate_text(&model, state_size, max_words)?;
    println!("Here is the generated text:\n\n{}", generated_text);
    Ok(())
}

fn generate_markov_chain(input: &str, state_size: usize) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    if state_size == 0 {
        return Err("State size must be greater than 0".into());
    }
    
    let mut input_vec: Vec<String> = Vec::new();
    let mut model: HashMap<String, Vec<String>> = HashMap::new();

    // Collect Words
    for line in input.lines() {
        for word in line.split_whitespace() {
            if !word.is_empty() {
                input_vec.push(word.to_string());
            }
        }
    }
    
    if input_vec.len() < state_size + 1 {
        return Err(format!("Input text has {} words, but need at least {} words for state size {}", 
                          input_vec.len(), state_size + 1, state_size).into());
    }
    
    println!("Collected {} words from input.", input_vec.len());

    // Cook up the Markov Chain with given state_size
    for i in state_size..(input_vec.len()) {
        let current_word = &input_vec[i];
        let previous_words = input_vec.get(i-state_size..i)
            .ok_or("Failed to get previous words slice")?
            .join(" ");
        if model.contains_key(&previous_words) {
            model.get_mut(&previous_words)
                .ok_or("Failed to get mutable reference to model entry")?
                .push(current_word.to_string());
        } else {
            model.insert(previous_words.clone(), vec![current_word.to_string()]);
        }
        println!("State: '{}', Next: '{}'", previous_words, current_word);
    }

    println!("Markov chain construction complete. States: {}", model.len());
    Ok(model)
}

fn generate_text(model: &HashMap<String, Vec<String>>, state_size: usize, max_words: usize) -> Result<String, Box<dyn std::error::Error>> {
    if max_words == 0 {
        return Err("Max words must be greater than 0".into());
    }
    
    let mut output_vec: Vec<String> = Vec::new();

    // Generate the text based off the markov model we generated

    // Get a valid starting character with a capital letter.
    let starter = get_text_starter(model, state_size)?;
    println!("Starter Chosen: {:#?}", starter);
    let starter_vec: Vec<&str> = starter.split(" ").collect();
    for i in starter_vec.iter() {
        output_vec.push(i.to_string());
    }

    // Get next state based on starter word
    for i in state_size..max_words {
        let previous_words = output_vec.get(i-state_size..i)
            .ok_or("Failed to get previous words slice")?
            .join(" ");
        println!("Current state: '{}'", previous_words);
        let next_word = match model.get(&previous_words) {
            Some(words) => {
                let chosen = words.choose(&mut rand::rng())
                    .ok_or("No words available for current state")?;
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
    Ok(output)
}

fn get_text_starter(model: &HashMap<String, Vec<String>>, state_size: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut starters_all: Vec<String> = Vec::new();

    // Checks for capital letters in the start of the state
    for (key, _value) in model.iter() {
        if let Some(first_char) = key.chars().next() {
            if first_char.is_uppercase() && first_char.is_alphabetic() {
                starters_all.push(key.to_string());
            }
        }
    }

    // Check that the last word in the state doesn't start with a capital letter (to prevent proper nouns)
    let mut starters_valid: Vec<String> = Vec::new();
    for starter in starters_all.iter() {
        let words: Vec<&str> = starter.split_whitespace().collect();
        if words.len() >= state_size {
            if let Some(last_word) = words.get(state_size - 1) {
                if let Some(first_char) = last_word.chars().next() {
                    if !first_char.is_uppercase() || !first_char.is_alphabetic() {
                        starters_valid.push(starter.to_string());
                    }
                }
            }
        }
    }

    // If no valid starters, use any starter with capital letter
    if starters_valid.is_empty() {
        starters_valid = starters_all;
    }

    // If still no starters, use any key from the model
    if starters_valid.is_empty() {
        if let Some((key, _)) = model.iter().next() {
            starters_valid.push(key.clone());
        } else {
            return Err("Model is empty, cannot generate text".into());
        }
    }

    // Randomly pick 1 to be the starter
    let mut random_num = rand::rng();
    let starter = starters_valid.choose(&mut random_num)
        .ok_or("Failed to choose a starter")?;
    
    println!("Starter selected: '{}'", starter);
    Ok(starter.clone())
}