use clap::Parser;
use replisp::{repl, file_exec};
use std::fs;
use std::path::Path;

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

    /// Compile to Java bytecode (.class file)
    #[arg(short, long)]
    compile: bool,

    /// Output file name for compilation (default: Main.class)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.file {
        Some(filename) => {
            if args.compile {
                // Compilation mode
                compile_file(&filename, args.output.as_deref(), args.debug);
            } else {
                // Execution mode
                if args.debug {
                    println!("Loading file: {}", filename);
                }

                file_exec::execute_file(&filename, args.debug);
            }
        }
        None => {
            if args.compile {
                eprintln!("Error: --compile requires a source file");
                std::process::exit(1);
            }

            if args.debug {
                println!("Starting REPL mode");
            }

            repl::run_repl(args.debug);
        }
    }
}

fn compile_file(input: &str, output: Option<&str>, debug: bool) {
    use replisp::{tokenizer, parser, Compiler};

    if debug {
        println!("Compiling file: {}", input);
    }

    // Read source file
    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Tokenize
    let tokens = match tokenizer::tokenize(&source) {
        Some(tokens) => tokens,
        None => {
            eprintln!("Tokenization error");
            std::process::exit(1);
        }
    };

    if debug {
        println!("Tokens: {:?}", tokens);
    }

    // Parse
    let exprs = match parser::parse(tokens) {
        Ok(exprs) => exprs,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    if debug {
        println!("Parsed expressions: {:?}", exprs);
    }

    // Determine class name from output file or use default
    let (output_file, class_name) = if let Some(out) = output {
        let path = Path::new(out);
        let class_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Main")
            .to_string();
        (out.to_string(), class_name)
    } else {
        ("Main.class".to_string(), "Main".to_string())
    };

    if debug {
        println!("Output file: {}", output_file);
        println!("Class name: {}", class_name);
    }

    // Compile to bytecode
    let mut compiler = Compiler::new(class_name);
    let bytecode = match compiler.compile(&exprs) {
        Ok(bc) => bc,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            std::process::exit(1);
        }
    };

    // Write to file
    if let Err(e) = fs::write(&output_file, bytecode) {
        eprintln!("Error writing output file: {}", e);
        std::process::exit(1);
    }

    println!("Successfully compiled to {}", output_file);
    println!("Run with: java {}", Path::new(&output_file).file_stem().unwrap().to_str().unwrap());
}