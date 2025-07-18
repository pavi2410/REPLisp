use std::io::{self, Write};
use crate::{tokenizer, parser, evaluator};

pub fn run_repl(debug: bool) {
    println!("Welcome to REPLisp!");
    println!("Type expressions to evaluate them.");
    println!("Type :quit or press Ctrl+C to exit.");
    
    let mut env = evaluator::Environment::new();
    
    loop {
        print!("replisp> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                if input == ":quit" || input == ":q" {
                    println!("Goodbye!");
                    break;
                }
                
                if debug {
                    println!("Input: {}", input);
                }
                
                let tokens = tokenizer::tokenize(input);

                if tokens.is_none() {
                    eprintln!("Error tokenizing input: {}", input);
                    continue;
                }
                let tokens = tokens.unwrap();
                
                if debug {
                    println!("Tokens ({} total):", tokens.len());
                    for (i, token) in tokens.iter().enumerate() {
                        println!("  {}: {:?}", i, token);
                    }
                    println!("---");
                }
                
                match parser::parse(tokens) {
                    Ok(expressions) => {
                        if debug {
                            println!("AST ({} expressions):", expressions.len());
                            for (i, expr) in expressions.iter().enumerate() {
                                println!("  {}: {:?}", i, expr);
                            }
                            println!("---");
                        }
                        
                        // Evaluate each expression
                        for expr in &expressions {
                            match evaluator::eval_expr(expr, &mut env) {
                                Ok(value) => {
                                    if debug {
                                        println!("Result: {}", value);
                                    } else {
                                        println!("{}", value);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Evaluation error: {}", err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Parse error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}