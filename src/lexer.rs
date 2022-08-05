#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Number(i32),
    String(String),
    Symbol(String),
}

pub fn lex(input: String) -> Vec<Token> {
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

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::lexer::Token::*;

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
                RParen,
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
                RParen,
            ]
        );
    }
}
