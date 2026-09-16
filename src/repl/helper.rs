use std::cell::RefCell;
use std::rc::Rc;

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Helper};

use crate::environment::Environment;

const SPECIALS: &[&str] = &[
    "cond", "def", "defn", "do", "else", "false", "if", "lambda", "quote", "true",
];

const COMMANDS: &[&str] = &[":q", ":quit"];

const SYMBOL_CHARS: &str = "+-*/%=<>!?_-:";

pub struct ReplHelper {
    pub env: Rc<RefCell<Environment>>,
}

impl ReplHelper {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }
}

impl Helper for ReplHelper {}

impl Hinter for ReplHelper {
    type Hint = String;
}

impl Highlighter for ReplHelper {}

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) = current_symbol(line, pos);
        let mut names: Vec<String> = SPECIALS
            .iter()
            .chain(COMMANDS.iter())
            .map(|s| (*s).to_string())
            .chain(self.env.borrow().bindings.keys().cloned())
            .filter(|name| name.starts_with(word))
            .collect();
        names.sort();
        names.dedup();

        let candidates = names
            .into_iter()
            .map(|name| Pair {
                display: name.clone(),
                replacement: name,
            })
            .collect();
        Ok((start, candidates))
    }
}

impl Validator for ReplHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(match balance(ctx.input()) {
            Balance::Complete => ValidationResult::Valid(None),
            Balance::Incomplete => ValidationResult::Incomplete,
            Balance::ExtraClose => ValidationResult::Invalid(Some("unmatched ')'".into())),
        })
    }
}

fn current_symbol(line: &str, pos: usize) -> (usize, &str) {
    let prefix = &line[..pos.min(line.len())];
    let start = prefix
        .char_indices()
        .rev()
        .find(|(_, c)| !is_symbol_char(*c))
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);
    (start, &line[start..pos.min(line.len())])
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(c)
}

enum Balance {
    Complete,
    Incomplete,
    ExtraClose,
}

fn balance(input: &str) -> Balance {
    let mut depth = 0i32;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                let mut closed = false;
                for c in chars.by_ref() {
                    if c == '"' {
                        closed = true;
                        break;
                    }
                }
                if !closed {
                    return Balance::Incomplete;
                }
            }
            ';' => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Balance::ExtraClose;
                }
            }
            _ => {}
        }
    }

    if depth > 0 {
        Balance::Incomplete
    } else {
        Balance::Complete
    }
}
