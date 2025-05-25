use std::fs;
use std::path::Path;

use clap::Parser as ClapParser;
use rustyline::{DefaultEditor, Result as RustylineResult};

use crate::lenv::Lenv;
use error::ReplispResult;
use lval::Lval;
use parser::eval_str;

mod error;
mod eval;
mod lenv;
mod lval;
mod parser;
mod test_parser;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filepath: Option<String>,
}

fn main() -> RustylineResult<()> {
    let args = Cli::parse();

    let mut global_env = Lenv::new(None, None);

    if let Some(filepath) = args.filepath {
        // Run a file
        let path = Path::new(&filepath);
        if !path.exists() {
            eprintln!("Error: File '{}' does not exist", filepath);
            std::process::exit(1);
        }

        let source = fs::read_to_string(path).expect("Error reading file");

        print_eval_result(eval_str(&mut global_env, source.as_str()));
    } else {
        // Start REPL
        repl(&mut global_env)?;
    }

    Ok(())
}

/// Run the REPLisp REPL (Read-Eval-Print Loop)
fn repl(env: &mut Lenv) -> RustylineResult<()> {
    println!("Welcome to REPLisp!");
    println!("Type an expression to evaluate it, or type :q to exit.");

    // Create a rustyline editor for better line editing experience
    let mut rl = DefaultEditor::new()?;

    // Load history if it exists
    let history_path = Path::new("history.txt");
    if history_path.exists() {
        let _ = rl.load_history(history_path);
    }

    loop {
        let readline = rl.readline("[^_^] ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                if line == ":q" {
                    println!("Goodbye!");
                    break;
                }

                // Add to history
                rl.add_history_entry(line.as_str())?;

                // Evaluate the input
                print_eval_result(eval_str(env, line.as_str()));
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    rl.save_history(history_path)?;

    Ok(())
}

fn print_eval_result(v: ReplispResult<Box<Lval>>) {
    match v {
        Ok(res) => println!("{res}"),
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
