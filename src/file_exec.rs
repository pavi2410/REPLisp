use std::fs;
use std::process;
use crate::{tokenizer, parser, evaluator, Environment};

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
    
    let expressions = match parser::parse(tokens) {
        Ok(expressions) => {
            if debug {
                println!("AST ({} expressions):", expressions.len());
                for (i, expr) in expressions.iter().enumerate() {
                    println!("  {}: {:?}", i, expr);
                }
                println!("---");
            }
            expressions
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };
    
    let mut env = Environment::new();
    
    for (i, expr) in expressions.iter().enumerate() {
        let mut stack = Vec::new();
        match evaluator::eval_expr_with_stack(expr, &mut env, &mut stack) {
            Ok(value) => {
                if debug {
                    println!("Expression {}: {} => {}", i, expr, value);
                }
                // Don't print results implicitly - only explicit print calls show output
            }
            Err(err) => {
                let error_with_stack = err.with_stack(&stack);
                eprintln!("Evaluation error in expression {}: {}", i, error_with_stack);
                process::exit(1);
            }
        }
    }
}