#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
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
    
    fn read_number(&mut self) -> Token {
        let start = self.position;

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
        
        Token::Number(number)
    }
    
    fn read_string(&mut self) -> Token {
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
        
        Token::String(string_content)
    }
    
    fn read_symbol(&mut self) -> Token {
        let start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || "+-*/%=<>!?_-".contains(ch) {
                self.advance();
            } else {
                break;
            }
        }
        
        let symbol: String = self.input[start..self.position].iter().collect();
        Token::Symbol(symbol)
    }
    
    fn read_comment(&mut self) -> Token {
        self.advance(); // Skip semicolon
        let start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
        
        let comment: String = self.input[start..self.position].iter().collect();
        Token::Comment(comment)
    }
    
    pub fn next_token(&mut self) -> Token {
        loop {
            match self.current_char {
                None => return Token::Eof,
                
                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                
                Some('(') => {
                    self.advance();
                    return Token::LeftParen;
                }
                
                Some(')') => {
                    self.advance();
                    return Token::RightParen;
                }
                
                Some('\'') => {
                    self.advance();
                    return Token::Quote;
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
                    let unknown_char = self.current_char.unwrap();
                    return Token::Unknown(unknown_char.to_string());
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
        let is_eof = token == Token::Eof;
        let is_unknown = matches!(token, Token::Unknown(_));

        if is_unknown {
            println!("tokenize: Found unknown character: {:?}", token);
            return None;
        }

        tokens.push(token);
        
        if is_eof {
            break;
        }
    }
    
    Some(tokens)
}