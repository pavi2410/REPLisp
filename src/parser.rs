use std::collections::vec_deque::VecDeque;
use std::fmt::{Display, Formatter};

use crate::Token;

#[derive(Debug, PartialEq)]
pub enum AstType {
    Number(i32),
    Symbol(String),
    String(String),
    Comment(String),
    Sexpr(VecDeque<AstType>),
    Qexpr(VecDeque<AstType>),
    Expr(Box<AstType>),
    Lisp(VecDeque<AstType>),
}

// impl Display for AstType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AstType::Number(n) => write!(f, "{}", n),
//             AstType::Symbol(s) => write!(f, "{}", s),
//             AstType::String(s) => write!(f, "\"{}\"", s),
//             AstType::Comment(s) => write!(f, "# {}", s),
//             AstType::Sexpr(e) => {
//                 write!(f, "(")?;
//                 self.children.iter().enumerate().map(|(i, obj)| {
//                     write!(f, "{:?}", obj)
//                 }).collect();
//                 write!(f, ")")
//             }
//             AstType::Qexpr(e) => {
//                 write!(f, "{{\n")?;
//                 self.children.iter().enumerate().map(|(i, obj)| {
//                     write!(f, "{:?}", obj)
//                 }).collect();
//                 write!(f, "}}\n")
//             }
//             AstType::Expr(k) => {
//                 self.children.iter().enumerate().map(|(i, obj)| {
//                     write!(f, "{:?}", obj)
//                 }).collect()
//             }
//             AstType::Lisp(e) => {
//                 self.children.iter().enumerate().map(|(i, obj)| {
//                     write!(f, "{:?}", obj)
//                 }).collect()
//             }
//         }
//     }
// }

pub fn parse(tokens: &mut VecDeque<Token>) -> AstType {
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() {
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    AstType::Lisp(nodes)
}

fn parse_string(tokens: &mut VecDeque<Token>) -> AstType {
    let s = if let Token::String(s) = tokens.pop_front().unwrap() { s } else { todo!() };
    AstType::String(s)
}

fn parse_symbol(tokens: &mut VecDeque<Token>) -> AstType {
    let s = if let Token::Symbol(s) = tokens.pop_front().unwrap() { s } else { todo!() };
    AstType::Symbol(s)
}

fn parse_number(tokens: &mut VecDeque<Token>) -> AstType {
    let n = if let Token::Number(n) = tokens.pop_front().unwrap() { n } else { todo!() };
    AstType::Number(n)
}

fn parse_sexpr(tokens: &mut VecDeque<Token>) -> AstType {
    tokens.pop_front();
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() && tokens.front() != Some(&Token::RParen) {
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    tokens.pop_front();
    AstType::Sexpr(nodes)
}

fn parse_qexpr(tokens: &mut VecDeque<Token>) -> AstType {
    tokens.pop_front();
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() && tokens.front() != Some(&Token::RBrace) {
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    tokens.pop_front();
    AstType::Qexpr(nodes)
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Option<AstType> {
    if let Some(token) = tokens.front() {
        match token {
            Token::Number(_) => Some(parse_number(tokens)),
            Token::Symbol(_) => Some(parse_symbol(tokens)),
            Token::String(_) => Some(parse_string(tokens)),
            Token::LParen => Some(parse_sexpr(tokens)),
            Token::RParen => None,
            Token::LBrace => Some(parse_qexpr(tokens)),
            Token::RBrace => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::vec_deque::VecDeque;

    use crate::lexer::Token::*;
    use crate::parser::{parse, AstType};

    #[test]
    fn test_parser() {
        let mut tokens = VecDeque::from([
            LParen,
            Symbol("+".to_string()),
            Number(1),
            Number(2),
            RParen,
        ]);
        let result = parse(&mut tokens);
        assert_eq!(
            result,
            AstType::Lisp(VecDeque::from([
                AstType::Sexpr(VecDeque::from([
                    AstType::Symbol("+".to_string()),
                    AstType::Number(1),
                    AstType::Number(2),
                ]))])));
    }
}
