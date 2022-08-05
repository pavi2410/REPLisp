use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use lexer::Token;

mod lexer;

/*
   GRAMMAR

   number : /-?[0-9]+/
   symbol : /[a-zA-Z0-9_+\-*\/\\=<>!&]+/
   string : '"' (\\.|[^\"])* '"'
   comment : ';' [^\r\n]*
   sexpr : '(' <expr>* ')'
   qexpr : '{' <expr>* '}'
   expr : <number> | <symbol> | <string>
        | <comment> | <sexpr> | <qexpr>
   lisp : <expr>*
*/

fn main() {
    println!("REPLisp v2.0");
    println!("Type :q to quit");

    loop {
        let input = prompt("> ");

        if input == ":q" {
            break;
        }

        let mut tokens = lexer::lex(input);

        println!("tokens -> {:?}", tokens);

        // let ast = parse(tokens);
        // let output = eval(ast);
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
