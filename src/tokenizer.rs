#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
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
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
            line: 1,
            column: 1,
        }
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn current_position(&self) -> Position {
        Position::new(self.line, self.column)
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
    
    fn read_number(&mut self) -> Token {
        let start = self.position;
        let pos = self.current_position();

        // Handle negative numbers
        if self.current_char == Some('-') {
            self.advance();
        }
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }
        
        let number_str: String = self.input[start..self.position].iter().collect();
        let number = number_str.parse::<f64>().unwrap_or(0.0);
        
        Token::new(TokenType::Number(number), pos)
    }
    
    fn read_string(&mut self) -> Token {
        let pos = self.current_position();
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
        let pos = self.current_position();
        
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
        let pos = self.current_position();
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
    
    pub fn next_token(&mut self) -> Token {
        loop {
            match self.current_char {
                None => return Token::new(TokenType::Eof, self.current_position()),
                
                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                
                Some('(') => {
                    let pos = self.current_position();
                    self.advance();
                    return Token::new(TokenType::LeftParen, pos);
                }
                
                Some(')') => {
                    let pos = self.current_position();
                    self.advance();
                    return Token::new(TokenType::RightParen, pos);
                }
                
                Some('\'') => {
                    let pos = self.current_position();
                    self.advance();
                    return Token::new(TokenType::Quote, pos);
                }
                
                Some('"') => {
                    return self.read_string();
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
                    return self.read_symbol();
                }

                Some(_) => {
                    // If we reach here, it's an unknown token
                    let pos = self.current_position();
                    let unknown_char = self.current_char.unwrap();
                    self.advance();
                    return Token::new(TokenType::Unknown(unknown_char.to_string()), pos);
                }
            }
        }
    }
}

pub fn tokenize(input: &str) -> Option<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = tokenizer.next_token();
        let is_eof = matches!(token.token_type, TokenType::Eof);
        let is_unknown = matches!(token.token_type, TokenType::Unknown(_));

        if is_unknown {
            println!("tokenize: Found unknown character at line {}, column {}: {:?}", 
                     token.position.line, token.position.column, token.token_type);
            return None;
        }

        tokens.push(token);
        
        if is_eof {
            break;
        }
    }
    
    Some(tokens)
}