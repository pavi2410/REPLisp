use std::collections::vec_deque::VecDeque;

use crate::Token;

#[derive(Debug, PartialEq)]
enum AstType {
    Number,
    Symbol,
    String,
    Comment,
    Sexpr,
    Qexpr,
    Expr,
    Lisp,
}

#[derive(Debug, PartialEq)]
pub struct AstNode {
    type_: AstType,
    children: Option<VecDeque<AstNode>>,
}

pub fn parse(tokens: &mut VecDeque<Token>) -> AstNode {
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() {
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    AstNode {
        type_: AstType::Lisp,
        children: Some(nodes),
    }
}

fn parse_string(tokens: &mut VecDeque<Token>) -> AstNode {
    tokens.pop_front();
    AstNode {
        type_: AstType::String,
        children: None,
    }
}

fn parse_symbol(tokens: &mut VecDeque<Token>) -> AstNode {
    tokens.pop_front();
    AstNode {
        type_: AstType::Symbol,
        children: None,
    }
}

fn parse_number(tokens: &mut VecDeque<Token>) -> AstNode {
    tokens.pop_front();
    AstNode {
        type_: AstType::Number,
        children: None,
    }
}

fn parse_sexpr(tokens: &mut VecDeque<Token>) -> AstNode {
    tokens.pop_front();
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() && tokens.front() != Some(&Token::RParen) {
        println!("loop -> {:?}", tokens.front());
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    tokens.pop_front();
    AstNode {
        type_: AstType::Sexpr,
        children: Some(nodes),
    }
}

fn parse_qexpr(tokens: &mut VecDeque<Token>) -> AstNode {
    tokens.pop_front();
    let mut nodes = VecDeque::new();
    while !tokens.is_empty() && tokens.front() != Some(&Token::RBrace) {
        println!("loop -> {:?}", tokens.front());
        if let Some(node) = parse_expr(tokens) {
            nodes.push_back(node);
        }
    }
    tokens.pop_front();
    AstNode {
        type_: AstType::Qexpr,
        children: Some(nodes),
    }
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Option<AstNode> {
    if let Some(token) = tokens.front() {
        match token {
            Token::Number(_n) => Some(AstNode {
                type_: AstType::Expr,
                children: Some(VecDeque::from([parse_number(tokens)])),
            }),
            Token::Symbol(_s) => Some(AstNode {
                type_: AstType::Expr,
                children: Some(VecDeque::from([parse_symbol(tokens)])),
            }),
            Token::String(_s) => Some(AstNode {
                type_: AstType::Expr,
                children: Some(VecDeque::from([parse_string(tokens)])),
            }),
            Token::LParen => Some(AstNode {
                type_: AstType::Expr,
                children: Some(VecDeque::from([parse_sexpr(tokens)])),
            }),
            Token::RParen => None,
            Token::LBrace => Some(AstNode {
                type_: AstType::Expr,
                children: Some(VecDeque::from([parse_qexpr(tokens)])),
            }),
            Token::RBrace => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::vec_deque::VecDeque;

    use crate::lexer::lex;
    use crate::lexer::Token::*;
    use crate::parser::{parse, AstNode, AstType};

    #[test]
    fn test_parser() {
        let mut tokens = VecDeque::from(vec![
            LParen,
            Symbol("+".to_string()),
            Number(1),
            Number(2),
            RParen,
        ]);
        let result = parse(&mut tokens);
        assert_eq!(
            result,
            AstNode {
                type_: AstType::Lisp,
                children: Some(VecDeque::from([AstNode {
                    type_: AstType::Expr,
                    children: Some(VecDeque::from([AstNode {
                        type_: AstType::Sexpr,
                        children: Some(VecDeque::from([
                            AstNode {
                                type_: AstType::Expr,
                                children: Some(VecDeque::from([AstNode {
                                    type_: AstType::Symbol,
                                    children: None,
                                }])),
                            },
                            AstNode {
                                type_: AstType::Expr,
                                children: Some(VecDeque::from([AstNode {
                                    type_: AstType::Number,
                                    children: None,
                                }])),
                            },
                            AstNode {
                                type_: AstType::Expr,
                                children: Some(VecDeque::from([AstNode {
                                    type_: AstType::Number,
                                    children: None,
                                }])),
                            }])),
                    }])),
                }])),
            }
        );
    }
}
