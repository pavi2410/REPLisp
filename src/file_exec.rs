use std::fs;
use std::process;
use crate::{tokenizer, parser};

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
    
    if tokens.is_none() {
        eprintln!("Error tokenizing file '{}'", filename);
        process::exit(1);
    }
    let tokens = tokens.unwrap();

    if debug {
        println!("Tokens ({} total):", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?}", i, token);
        }
        println!("---");
    }
    
    println!("Parsing: {}", filename);
    match parser::parse(tokens) {
        Ok(expressions) => {
            if debug {
                println!("AST ({} expressions):", expressions.len());
                for (i, expr) in expressions.iter().enumerate() {
                    println!("  {}: {:?}", i, expr);
                }
                println!("---");
                println!("Pretty printed:");
                for (i, expr) in expressions.iter().enumerate() {
                    println!("  {}: {}", i, expr);
                }
            } else {
                println!("AST: {:?}", expressions);
            }
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    }
}