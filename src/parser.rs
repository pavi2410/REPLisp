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
    pub expr_type: ExprType,
    pub span: Span,
}

impl Expr {
    pub fn new(expr_type: ExprType, span: Span) -> Self {
        Self { expr_type, span }
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
                    loc.line, loc.column, token.token_type
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
    
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();
        
        while self.current_token().is_some() {
            expressions.push(self.parse_expression()?);
        }
        
        Ok(expressions)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Some(token) => {
                let span = token.span;
                match &token.token_type {
                    TokenType::Number(n) => {
                        let num = *n;
                        self.advance();
                        Ok(Expr::new(ExprType::Number(num), span))
                    }
                    
                    TokenType::String(s) => {
                        let string = s.clone();
                        self.advance();
                        Ok(Expr::new(ExprType::String(string), span))
                    }
                    
                    TokenType::Symbol(s) => {
                        let symbol = s.clone();
                        self.advance();
                        Ok(Expr::new(ExprType::Symbol(symbol), span))
                    }
                    
                    TokenType::Quote => {
                        self.advance();
                        let inner = self.parse_expression()?;
                        let end = inner.span.end;
                        Ok(Expr::new(ExprType::Quote(Box::new(inner)), Span::new(span.start, end)))
                    }
                    
                    TokenType::LeftParen => {
                        self.advance();
                        self.parse_list(span)
                    }
                    
                    _ => Err(ParseError::UnexpectedToken(token.clone())),
                }
            }
            None => Err(ParseError::UnexpectedEof),
        }
    }
    
    fn parse_list(&mut self, open: Span) -> Result<Expr, ParseError> {
        let mut elements = Vec::new();
        
        loop {
            match self.current_token() {
                Some(token) => {
                    match &token.token_type {
                        TokenType::RightParen => {
                            let end = token.span.end;
                            self.advance();
                            return Ok(Expr::new(
                                ExprType::List(elements),
                                Span::new(open.start, end),
                            ));
                        }
                        
                        _ => {
                            let expr = self.parse_expression()?;
                            elements.push(expr);
                        }
                    }
                }
                
                None => {
                    return Err(ParseError::UnmatchedParen(open));
                }
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
                write!(f, "Unexpected token at offset {}: {:?}", token.span.start, token.token_type)
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
        match &self.expr_type {
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
