use std::io::{self, Write};

pub fn run_repl(debug: bool) {
    println!("Welcome to REPLisp!");
    println!("Type expressions to evaluate them.");
    println!("Type :quit or press Ctrl+C to exit.");
    
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
                
                // TODO: Tokenize and evaluate the input
                println!("Echo: {}", input);
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}