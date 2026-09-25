use std::fmt::Write;

use crate::tokenizer::{Token, TokenType, Span, LineIndex};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    Quote(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprType,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprType, span: Span) -> Self {
        Self { kind, span }
    }

    fn kind_label(&self) -> String {
        match &self.kind {
            ExprType::Number(n) => format!("Number({n})"),
            ExprType::String(s) => format!("String({s:?})"),
            ExprType::Symbol(s) => format!("Symbol({s})"),
            ExprType::List(_) => "List".to_string(),
            ExprType::Quote(_) => "Quote".to_string(),
        }
    }

    /// Debug tree with spans, e.g. `List [0..11]` / `├─ Symbol(+) [1..2]`.
    pub fn debug_tree(&self) -> String {
        let mut out = String::new();
        self.write_debug_tree(&mut out, "  ", true, true);
        out
    }

    fn write_debug_tree(&self, out: &mut String, prefix: &str, is_last: bool, is_root: bool) {
        let connector = if is_root {
            ""
        } else if is_last {
            "└─ "
        } else {
            "├─ "
        };

        let _ = writeln!(out, "{prefix}{connector}{:<12} {}", self.kind_label(), self.span);

        let children: Vec<&Expr> = match &self.kind {
            ExprType::List(elems) => elems.iter().collect(),
            ExprType::Quote(inner) => vec![inner.as_ref()],
            _ => return,
        };

        let child_prefix = if is_root {
            format!("{prefix}  ")
        } else if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        let last = children.len().saturating_sub(1);
        for (i, child) in children.into_iter().enumerate() {
            child.write_debug_tree(out, &child_prefix, i == last, false);
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken(Token),
    UnmatchedParen(Span),
}

impl ParseError {
    pub fn display(&self, source: &str) -> String {
        let lines = LineIndex::new(source);
        match self {
            ParseError::UnexpectedEof => "Unexpected end of input".to_string(),
            ParseError::UnexpectedToken(token) => {
                let loc = lines.position(token.span.start);
                format!(
                    "Unexpected token at line {}, column {}: {:?}",
                    loc.line, loc.column, token.kind
                )
            }
            ParseError::UnmatchedParen(span) => {
                let loc = lines.position(span.start);
                format!(
                    "Unmatched parenthesis at line {}, column {}",
                    loc.line, loc.column
                )
            }
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn eat(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned()?;
        self.position += 1;
        Some(token)
    }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();
        
        while self.peek().is_some() {
            expressions.push(self.parse_expression()?);
        }
        
        Ok(expressions)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let Token { kind, span } = self.eat().ok_or(ParseError::UnexpectedEof)?;
        match kind {
            TokenType::Number(n) => Ok(Expr::new(ExprType::Number(n), span)),
            TokenType::String(s) => Ok(Expr::new(ExprType::String(s), span)),
            TokenType::Symbol(s) => Ok(Expr::new(ExprType::Symbol(s), span)),
            TokenType::Quote => {
                let inner = self.parse_expression()?;
                let end = inner.span.end;
                Ok(Expr::new(ExprType::Quote(Box::new(inner)), Span::new(span.start, end)))
            }
            TokenType::LeftParen => self.parse_list(span),
            kind => Err(ParseError::UnexpectedToken(Token { kind, span })),
        }
    }
    
    fn parse_list(&mut self, open: Span) -> Result<Expr, ParseError> {
        let mut elements = Vec::new();
        
        loop {
            match self.peek() {
                None => return Err(ParseError::UnmatchedParen(open)),
                Some(token) if matches!(token.kind, TokenType::RightParen) => {
                    let end = token.span.end;
                    self.eat();
                    return Ok(Expr::new(
                        ExprType::List(elements),
                        Span::new(open.start, end),
                    ));
                }
                Some(_) => elements.push(self.parse_expression()?),
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParseError> {
    Parser::new(tokens).parse()
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::UnexpectedToken(token) => {
                write!(f, "Unexpected token at offset {}: {:?}", token.span.start, token.kind)
            }
            ParseError::UnmatchedParen(span) => {
                write!(f, "Unmatched parenthesis at offset {}", span.start)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ExprType::Number(n) => write!(f, "{}", n),
            ExprType::String(s) => write!(f, "\"{}\"", s),
            ExprType::Symbol(s) => write!(f, "{}", s),
            ExprType::Quote(expr) => write!(f, "'{}", expr),
            ExprType::List(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
        }
    }
}
