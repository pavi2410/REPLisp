extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::vec_deque::VecDeque;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use clap::Parser as ClapParser;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser as PestParser;

use lexer::Token;

mod lexer;
mod parser;
mod eval;

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
    loop {
        let input = prompt("> ");

        if input == ":q" {
            break;
        }

        match ReplispParser::parse(Rule::expr, &input) {
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
