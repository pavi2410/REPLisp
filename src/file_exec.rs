use std::fs;
use std::process;

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
    
    // TODO: Tokenize and evaluate the content
    println!("Executing: {}", filename);
}