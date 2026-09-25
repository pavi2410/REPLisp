use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};

use crate::environment::Environment;
use crate::{evaluator, parser, tokenizer};

mod helper;

use helper::ReplHelper;

pub fn run_repl(debug: bool) {
    println!("Welcome to REPLisp!");
    println!("Type expressions to evaluate them.");
    println!("Type :quit or Ctrl+D to exit.");

    let env = Rc::new(RefCell::new(Environment::new()));
    let config = Config::builder()
        .auto_add_history(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl = match Editor::<ReplHelper, DefaultHistory>::with_config(config) {
        Ok(mut editor) => {
            editor.set_helper(Some(ReplHelper::new(Rc::clone(&env))));
            editor
        }
        Err(err) => {
            eprintln!("Failed to start REPL: {err}");
            return;
        }
    };

    let history = history_path();
    if let Some(path) = &history {
        let _ = rl.load_history(path);
    }

    loop {
        match rl.readline("replisp> ") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                if input == ":quit" || input == ":q" {
                    println!("Goodbye!");
                    break;
                }
                eval_input(input, &env, debug);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {err}");
                break;
            }
        }
    }

    if let Some(path) = &history {
        let _ = rl.save_history(path);
    }
}

fn eval_input(input: &str, env: &Rc<RefCell<Environment>>, debug: bool) {
    if debug {
        println!("Input: {input}");
    }

    let tokens = match tokenizer::tokenize(input) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Error tokenizing input: {}", err.display(input));
            return;
        }
    };

    if debug {
        println!("Tokens ({}):", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  {i:<2} {token}");
        }
        println!();
    }

    match parser::parse(tokens) {
        Ok(expressions) => {
            if debug {
                println!("AST ({} expr{}):", expressions.len(), if expressions.len() == 1 { "" } else { "s" });
                for expr in &expressions {
                    print!("{}", expr.debug_tree());
                }
                println!();
            }

            let mut env = env.borrow_mut();
            for expr in &expressions {
                let mut stack = Vec::new();
                match evaluator::eval_expr_with_stack(expr, &mut env, &mut stack) {
                    Ok(value) => {
                        if debug {
                            println!("Result: {value}");
                        } else {
                            println!("{value}");
                        }
                    }
                    Err(err) => {
                        let error_with_stack = err.with_stack(&stack);
                        eprintln!("Evaluation error: {}", error_with_stack.display(input));
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Parse error: {}", err.display(input));
        }
    }
}

fn history_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(PathBuf::from(home).join(".replisp_history"))
}
