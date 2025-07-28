use crate::tokenizer::{Token, TokenType, Position};

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
    pub position: Position,
}

impl Expr {
    pub fn new(expr_type: ExprType, position: Position) -> Self {
        Self { expr_type, position }
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
    UnmatchedParen(Position),
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
    
    // fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
    //     match self.current_token() {
    //         Some(token) if *token == expected => {
    //             self.advance();
    //             Ok(())
    //         }
    //         Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
    //         None => Err(ParseError::UnexpectedEof),
    //     }
    // }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();
        
        while let Some(token) = self.current_token() {
            match &token.token_type {
                TokenType::Eof => break,
                _ => {
                    let expr = self.parse_expression()?;
                    expressions.push(expr);
                }
            }
        }
        
        Ok(expressions)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Some(token) => {
                let pos = token.position.clone();
                match &token.token_type {
                    TokenType::Number(n) => {
                        let num = *n;
                        self.advance();
                        Ok(Expr::new(ExprType::Number(num), pos))
                    }
                    
                    TokenType::String(s) => {
                        let string = s.clone();
                        self.advance();
                        Ok(Expr::new(ExprType::String(string), pos))
                    }
                    
                    TokenType::Symbol(s) => {
                        let symbol = s.clone();
                        self.advance();
                        Ok(Expr::new(ExprType::Symbol(symbol), pos))
                    }
                    
                    TokenType::Quote => {
                        self.advance();
                        let expr = self.parse_expression()?;
                        Ok(Expr::new(ExprType::Quote(Box::new(expr)), pos))
                    }
                    
                    TokenType::LeftParen => {
                        self.advance();
                        self.parse_list(pos)
                    }
                    
                    _ => Err(ParseError::UnexpectedToken(token.clone())),
                }
            }
            None => Err(ParseError::UnexpectedEof),
        }
    }
    
    fn parse_list(&mut self, start_pos: Position) -> Result<Expr, ParseError> {
        let mut elements = Vec::new();
        
        loop {
            match self.current_token() {
                Some(token) => {
                    match &token.token_type {
                        TokenType::RightParen => {
                            self.advance();
                            break;
                        }
                        
                        TokenType::Eof => {
                            return Err(ParseError::UnmatchedParen(start_pos));
                        }
                        
                        _ => {
                            let expr = self.parse_expression()?;
                            elements.push(expr);
                        }
                    }
                }
                
                None => {
                    return Err(ParseError::UnexpectedEof);
                }
            }
        }
        
        Ok(Expr::new(ExprType::List(elements), start_pos))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::UnexpectedToken(token) => {
                write!(f, "Unexpected token at line {}, column {}: {:?}", 
                       token.position.line, token.position.column, token.token_type)
            }
            ParseError::UnmatchedParen(pos) => {
                write!(f, "Unmatched parenthesis at line {}, column {}", pos.line, pos.column)
            }
        }
    }
}

impl std::error::Error for ParseError {}

// Pretty printing for expressions
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