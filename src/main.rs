extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use clap::Parser as ClapParser;
use error::ReplispResult;
use lval::Lval;
use parse::eval_str;
use crate::lenv::Lenv;

mod test_parser;
mod eval;
mod lval;
mod lenv;
mod error;
mod parse;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filepath: Option<String>,
}



fn main() {
    let args = Cli::parse();

    let mut global_env = Lenv::new(None, None);

    if let Some(filepath) = args.filepath {
        let source = std::fs::read_to_string(filepath).unwrap();
        print_eval_result(eval_str(&mut global_env, source.as_str()));
    } else {
        repl(&mut global_env);
    }
}



fn repl(env: &mut Lenv) {
    println!("Welcome to Replisp!");
    println!("Type an expression to evaluate it, or type :q to exit.");


    loop {
        let input = prompt("[^_^] ");

        if input == ":q" {
            break;
        }

        print_eval_result(eval_str(env, input.as_str()));
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

fn print_eval_result(v: ReplispResult<Box<Lval>>) {
	match v {
		Ok(res) => println!("{res}"),
		Err(e) => eprintln!("Error: {e:?}"),
	}
}