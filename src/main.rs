use std::io::Write;
use std::io::stdin;
use std::io::stdout;

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

        let tokens = tokenize(input);
        // println!("{:?}", tokens);
        for t in tokens {
            println!("Token({:?}, {})", t.kind, t.val);
        }
        // let ast = parse(tokens);
        // let output = eval(ast);
    }
}

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    stdout().flush().unwrap();
    stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    line.trim().to_string()
}

#[derive(Debug)]
enum TokenType {
    number,
    symbol,
    string,
    comment,
    lpar, rpar,
    lbrace, rbrace,
}

struct Token<'a> {
    kind: TokenType,
    val: &'a str
}

fn tokenize(input: String) -> Vec<Token<'static>> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    use TokenType::*;

    for c in input.chars() {
        buf.push(c);
        let token = match buf.as_str() {
            "(" => Some(Token { kind: lpar, val: "(" }),
            ")" => Some(Token { kind: rpar, val: ")" }),
            "{" => Some(Token { kind: rpar, val: "{" }),
            "}" => Some(Token { kind: rpar, val: "}" }),
            "ab" => Some(Token { kind: symbol, val: "ab" }),
            _ => None
        };
        if let Some(token) = token {
            tokens.push(token);
            buf.clear();
        }
    }

    tokens
}

fn parse() {
    println!("hi");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
