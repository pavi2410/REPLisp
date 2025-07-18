use clap::Parser;

#[derive(Parser)]
#[command(name = "replisp")]
#[command(about = "A modern Lisp-inspired programming language")]
#[command(version = "0.1.0")]
struct Args {
    /// Path to the REPLisp source file to execute
    file: Option<String>,
    
    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    
    match args.file {
        Some(filename) => {
            if args.debug {
                println!("Loading file: {}", filename);
            }
            
            println!("Executing file: {}", filename);
        }
        None => {
            if args.debug {
                println!("Starting REPL mode");
            }
            
            println!("Welcome to REPLisp!");
        }
    }
}