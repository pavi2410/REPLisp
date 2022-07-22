use std::io::stdin;
use std::io::stdout;
use std::io::Write;

/*
   GRAMMAR

   number : /-?[0-9]+/
   symbol : /[a-zA-Z0-9_+\-*\/\\=<>!&]+/
   string : '"' (\\.|[^\"])* '"'
   comment : ';' [^\r\n]*
   sexpr : '(' <expr>* ')'
   qexpr : '{' <expr>* '}'
   expr : <number> | <symbol> | <string>"
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

        let tokens = lex(input);

        println!("{:?}", tokens);

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

#[derive(Debug, PartialEq)]
enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Number(i32),
    String(String),
    Symbol(String),
}

fn lex(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let sanitized_input = &input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("{", " { ")
        .replace("}", " } ");

    for word in sanitized_input.split_ascii_whitespace() {
        let token = match word {
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),
            "{" => Some(Token::LBrace),
            "}" => Some(Token::RBrace),
            _ => {
                if let Ok(num) = word.parse::<i32>() {
                    Some(Token::Number(num))
                } else if word.starts_with('"') {
                    Some(Token::String(word[1..word.len() - 1].to_string()))
                } else {
                    Some(Token::Symbol(word.to_string()))
                }
            }
        };
        tokens.push(token.unwrap());
    }

    tokens
}

fn parse() {
    println!("hi");
}

#[cfg(test)]
mod tests {
    use crate::lex;
    use crate::Token::*;

    #[test]
    fn test_lexer() {
        let result = lex("(+ 2 24)".to_string());
        assert_eq!(
            result,
            vec![
                LParen,
                Symbol("+".to_string()),
                Number(2),
                Number(24),
                RParen
            ]
        );
    }

    #[test]
    fn test_lexer2() {
        let result = lex(r#"
        (fun {areacircle rad} { do
            (print "Radius:\t" rad)
            (print "Area:\t" (* rad rad))
        })
        "#
        .to_string());
        assert_eq!(
            result,
            vec![
                LParen,
                Symbol("fun".to_string()),
                LBrace,
                Symbol("areacircle".to_string()),
                Symbol("rad".to_string()),
                RBrace,
                LBrace,
                Symbol("do".to_string()),
                LParen,
                Symbol("print".to_string()),
                String("Radius:\\t".to_string()),
                Symbol("rad".to_string()),
                RParen,
                LParen,
                Symbol("print".to_string()),
                String("Area:\\t".to_string()),
                LParen,
                Symbol("*".to_string()),
                Symbol("rad".to_string()),
                Symbol("rad".to_string()),
                RParen,
                RParen,
                RBrace,
                RParen
            ]
        );
    }
}
