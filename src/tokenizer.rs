#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn start() -> Self {
        Self::new(1, 1)
    }

    pub fn advance(&mut self, ch: char) {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Symbol(String),
    
    // Delimiters
    LeftParen,
    RightParen,
    
    // Special
    Quote,
    
    // Whitespace and comments (usually ignored)
    Whitespace,
    Comment(String),
    
    // End of input
    Eof,

    // Unknown token
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Self {
        Self { token_type, position }
    }
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    loc: Position,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
            loc: Position::start(),
        }
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            self.loc.advance(ch);
        }
        
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_number(&mut self) -> Result<Token, TokenizeError> {
        let start = self.position;
        let pos = self.loc;

        if self.current_char == Some('-') {
            self.advance();
        }

        let mut seen_dot = false;
        let mut extra_dot = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' {
                if seen_dot {
                    extra_dot = true;
                }
                seen_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let number_str: String = self.input[start..self.position].iter().collect();
        if extra_dot {
            return Err(TokenizeError::InvalidNumber(number_str, pos));
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(Token::new(TokenType::Number(number), pos)),
            Err(_) => Err(TokenizeError::InvalidNumber(number_str, pos)),
        }
    }
    
    fn read_string(&mut self) -> Token {
        let pos = self.loc;
        self.advance(); // Skip opening quote
        let start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                break;
            }
            // TODO: Handle escape sequences
            self.advance();
        }
        
        let string_content: String = self.input[start..self.position].iter().collect();
        
        if self.current_char == Some('"') {
            self.advance(); // Skip closing quote
        }
        
        Token::new(TokenType::String(string_content), pos)
    }
    
    fn read_symbol(&mut self) -> Token {
        let start = self.position;
        let pos = self.loc;
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || "+-*/%=<>!?_-".contains(ch) {
                self.advance();
            } else {
                break;
            }
        }
        
        let symbol: String = self.input[start..self.position].iter().collect();
        Token::new(TokenType::Symbol(symbol), pos)
    }
    
    fn read_comment(&mut self) -> Token {
        let pos = self.loc;
        self.advance(); // Skip semicolon
        let start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
        
        let comment: String = self.input[start..self.position].iter().collect();
        Token::new(TokenType::Comment(comment), pos)
    }
    
    pub fn next_token(&mut self) -> Result<Token, TokenizeError> {
        loop {
            match self.current_char {
                None => return Ok(Token::new(TokenType::Eof, self.loc)),
                
                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                
                Some('(') => {
                    let pos = self.loc;
                    self.advance();
                    return Ok(Token::new(TokenType::LeftParen, pos));
                }
                
                Some(')') => {
                    let pos = self.loc;
                    self.advance();
                    return Ok(Token::new(TokenType::RightParen, pos));
                }
                
                Some('\'') => {
                    let pos = self.loc;
                    self.advance();
                    return Ok(Token::new(TokenType::Quote, pos));
                }
                
                Some('"') => {
                    return Ok(self.read_string());
                }
                
                Some(';') => {
                    let _comment = self.read_comment();
                    // Skip comments and continue
                    continue;
                }
                
                Some(ch) if ch.is_ascii_digit() => {
                    return self.read_number();
                }
                
                Some(ch) if ch == '-' && self.peek().map_or(false, |p| p.is_ascii_digit()) => {
                    return self.read_number();
                }
                
                Some(ch) if ch.is_alphanumeric() || "+-*/%=<>!?_-".contains(ch) => {
                    return Ok(self.read_symbol());
                }

                Some(_) => {
                    // If we reach here, it's an unknown token
                    let pos = self.loc;
                    let unknown_char = self.current_char.unwrap();
                    self.advance();
                    return Ok(Token::new(TokenType::Unknown(unknown_char.to_string()), pos));
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenizeError {
    Unknown(Token),
    InvalidNumber(String, Position),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = tokenizer.next_token()?;
        let is_eof = matches!(token.token_type, TokenType::Eof);

        if matches!(token.token_type, TokenType::Unknown(_)) {
            return Err(TokenizeError::Unknown(token));
        }

        tokens.push(token);
        
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::Unknown(token) => {
                write!(
                    f,
                    "Unknown character at line {}, column {}: {:?}",
                    token.position.line, token.position.column, token.token_type
                )
            }
            TokenizeError::InvalidNumber(lexeme, pos) => {
                write!(
                    f,
                    "Invalid number '{}' at line {}, column {}",
                    lexeme, pos.line, pos.column
                )
            }
        }
    }
}

impl std::error::Error for TokenizeError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn numbers(input: &str) -> Vec<f64> {
        tokenize(input)
            .unwrap()
            .into_iter()
            .filter_map(|t| match t.token_type {
                TokenType::Number(n) => Some(n),
                TokenType::Eof => None,
                other => panic!("unexpected token {other:?}"),
            })
            .collect()
    }

    fn invalid_number(input: &str) -> String {
        match tokenize(input) {
            Err(TokenizeError::InvalidNumber(lexeme, _)) => lexeme,
            other => panic!("expected InvalidNumber, got {other:?}"),
        }
    }

    #[test]
    fn parses_valid_numbers() {
        assert_eq!(numbers("42"), vec![42.0]);
        assert_eq!(numbers("3.14"), vec![3.14]);
        assert_eq!(numbers("-2.5"), vec![-2.5]);
        assert_eq!(numbers("0"), vec![0.0]);
        assert_eq!(numbers("1.0"), vec![1.0]);
    }

    #[test]
    fn rejects_multiple_dots() {
        assert_eq!(invalid_number("1.2.3"), "1.2.3");
        assert_eq!(invalid_number("-1.2.3"), "-1.2.3");
        assert_eq!(invalid_number("1..2"), "1..2");
        assert_eq!(invalid_number("1.2."), "1.2.");
    }

    #[test]
    fn invalid_number_includes_position() {
        let err = tokenize("(+ 1.2.3)").unwrap_err();
        match err {
            TokenizeError::InvalidNumber(lexeme, pos) => {
                assert_eq!(lexeme, "1.2.3");
                assert_eq!(pos, Position::new(1, 4));
            }
            other => panic!("expected InvalidNumber, got {other:?}"),
        }
    }
}