use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    Quote(Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken(Token),
    UnmatchedParen,
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
    
    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.current_token() {
            Some(token) if *token == expected => {
                self.advance();
                Ok(())
            }
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEof),
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();
        
        while let Some(token) = self.current_token() {
            match token {
                Token::Eof => break,
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
            Some(Token::Number(n)) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            
            Some(Token::String(s)) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::String(string))
            }
            
            Some(Token::Symbol(s)) => {
                let symbol = s.clone();
                self.advance();
                Ok(Expr::Symbol(symbol))
            }
            
            Some(Token::Quote) => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Expr::Quote(Box::new(expr)))
            }
            
            Some(Token::LeftParen) => {
                self.advance();
                self.parse_list()
            }
            
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEof),
        }
    }
    
    fn parse_list(&mut self) -> Result<Expr, ParseError> {
        let mut elements = Vec::new();
        
        loop {
            match self.current_token() {
                Some(Token::RightParen) => {
                    self.advance();
                    break;
                }
                
                Some(Token::Eof) => {
                    return Err(ParseError::UnmatchedParen);
                }
                
                Some(_) => {
                    let expr = self.parse_expression()?;
                    elements.push(expr);
                }
                
                None => {
                    return Err(ParseError::UnexpectedEof);
                }
            }
        }
        
        Ok(Expr::List(elements))
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
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParseError::UnmatchedParen => write!(f, "Unmatched parenthesis"),
        }
    }
}

impl std::error::Error for ParseError {}

// Pretty printing for expressions
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Quote(expr) => write!(f, "'{}", expr),
            Expr::List(elements) => {
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