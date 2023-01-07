extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use clap::Parser as ClapParser;
use pest::Parser as PestParser;

mod test_parser;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filepath: Option<String>,
}

#[derive(Parser)]
#[grammar = "replisp_grammar.pest"] // relative to src
struct ReplispParser;

fn main() {
    let args = Cli::parse();

    if let Some(filepath) = args.filepath {
        let contents = std::fs::read_to_string(filepath).unwrap();
        ReplispParser::parse(Rule::program, &contents).unwrap();
    } else {
        repl();
    }
}

fn repl() {
    println!("Welcome to Replisp!");
    println!("Type an expression to evaluate it, or type :q to exit.");
    loop {
        let input = prompt("(★‿★)> ");

        if input == ":q" {
            break;
        }

        match ReplispParser::parse(Rule::program, &input) {
            Ok(ast) => {
                println!("{:?}", ast);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    line.trim().to_string()
}
