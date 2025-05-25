use chumsky::prelude::*;
use std::fmt;

use crate::error::{Error, ReplispResult};
use crate::lval::{sexpr, Lval};

// Create an empty S-expression
fn sexpr_val() -> Lval {
    Lval::Sexpr(Vec::new())
}

// Create an empty Q-expression
fn qexpr_val() -> Lval {
    Lval::Qexpr(Vec::new())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Symbol(String),
    Number(i64),
    String(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // Symbols
    let symbol = filter(|c: &char| c.is_alphanumeric() || "+-*/\\%^=<>!&_".contains(*c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Symbol);

    // Numbers
    let number = text::int(10).map(|s: String| Token::Number(s.parse().unwrap()));

    // Strings with escape sequences
    let escape = just('\\').ignore_then(choice((
        just('\"').to('\"'),
        just('\\').to('\\'),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
    )));

    let string = just('\"')
        .ignore_then(
            choice((escape, filter(|c| *c != '\"' && *c != '\\')))
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just('\"'))
        .map(Token::String);

    // Special characters
    let lparen = just('(').map(|_| Token::LParen);
    let rparen = just(')').map(|_| Token::RParen);
    let lbrace = just('{').map(|_| Token::LBrace);
    let rbrace = just('}').map(|_| Token::RBrace);

    // Comments
    let comment = just('#').then(take_until(just('\n'))).padded();

    // Combine all token parsers
    let token = choice((lparen, rparen, lbrace, rbrace, string, number, symbol));

    // Whitespace and comments are ignored
    let tokens = token
        .padded()
        .repeated()
        .collect()
        .then_ignore(comment.repeated())
        .then_ignore(end());

    tokens
}

pub fn parser() -> impl Parser<Token, Lval, Error = Simple<Token>> {
    recursive(|expr| {
        let atom = select! {
            Token::Number(n) => Lval::Num(n),
            Token::Symbol(s) => Lval::Sym(s.clone()),
            Token::String(s) => Lval::Str(s.clone()), // Use Str variant for strings
        };

        let sexpr_inner = expr.clone().repeated().collect::<Vec<_>>().map(|exprs| {
            let mut s = sexpr_val();
            if let Lval::Sexpr(ref mut cells) = s {
                for e in exprs {
                    cells.push(Box::new(e));
                }
            }
            s
        });

        let qexpr_inner = expr.clone().repeated().collect::<Vec<_>>().map(|exprs| {
            let mut q = qexpr_val();
            if let Lval::Qexpr(ref mut cells) = q {
                for e in exprs {
                    cells.push(Box::new(e));
                }
            }
            q
        });

        let s_expression = sexpr_inner.delimited_by(just(Token::LParen), just(Token::RParen));

        let q_expression = qexpr_inner.delimited_by(just(Token::LBrace), just(Token::RBrace));

        choice((atom, s_expression, q_expression))
    })
}

/// Parse a string of REPLisp code into an Lval
pub fn parse(input: &str) -> ReplispResult<Box<Lval>> {
    // First, tokenize the input
    let tokens = lexer()
        .parse(input)
        .map_err(|e| Error::Parse(format!("Lexer error: {:?}", e)))?;

    // If there are no tokens, return an empty S-expression
    if tokens.is_empty() {
        return Ok(sexpr());
    }

    // Then parse the tokens into an Lval
    let result = parser()
        .parse(tokens.clone())
        .map_err(|e| Error::Parse(format!("Parser error: {:?}", e)))?;

    // Convert Lval to Box<Lval>
    Ok(Box::new(result))
}

/// Parse and evaluate a string of REPLisp code
pub fn eval_str(env: &mut crate::lenv::Lenv, source: &str) -> ReplispResult<Box<Lval>> {
    // Log the source code if debug mode is enabled
    if crate::debug::is_debug_enabled() {
        crate::debug_print!("Evaluating source:\n{}", source);
    }

    // Try to parse the entire source as a single expression first
    match parse(source) {
        Ok(ast) => {
            if crate::debug::is_debug_enabled() {
                crate::debug_print!("Parsed AST: {:?}", ast);
            }
            return crate::eval::eval(env, ast);
        }
        Err(e) => {
            if crate::debug::is_debug_enabled() {
                crate::debug_print!("Failed to parse entire source: {:?}", e);
                crate::debug_print!("Trying to parse line by line");
            }

            // If that fails, try to evaluate each line separately
            let mut result = crate::lval::sexpr();

            // Process each line that looks like a complete expression
            for line in source.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue; // Skip empty lines and comments
                }

                if crate::debug::is_debug_enabled() {
                    crate::debug_print!("Parsing line: {}", trimmed);
                }

                // Only try to parse lines that look like complete expressions
                if trimmed.starts_with('(') {
                    match parse(trimmed) {
                        Ok(ast) => {
                            if crate::debug::is_debug_enabled() {
                                crate::debug_print!("Successfully parsed line: {:?}", ast);
                            }
                            result = crate::eval::eval(env, ast)?;
                        }
                        Err(e) => {
                            eprintln!("Error parsing: {}", trimmed);
                            if crate::debug::is_debug_enabled() {
                                crate::debug_print!("Parse error: {:?}", e);
                            }
                            return Err(e);
                        }
                    }
                }
            }

            Ok(result)
        }
    }
}
