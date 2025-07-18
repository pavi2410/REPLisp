use std::fs;
use std::process;
use crate::tokenizer;

pub fn execute_file(filename: &str, debug: bool) {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };
    
    if debug {
        println!("File content ({} chars):", content.len());
        println!("{}", content);
        println!("---");
    }
    
    println!("Tokenizing: {}", filename);
    let tokens = tokenizer::tokenize(&content);
    
    if debug {
        println!("Tokens ({} total):", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?}", i, token);
        }
        println!("---");
    } else {
        println!("Tokens: {:?}", tokens);
    }
}