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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn point(offset: usize) -> Self {
        Self::new(offset, offset)
    }
}

#[derive(Debug, Clone)]
pub struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut starts = vec![0];
        for (i, ch) in source.chars().enumerate() {
            if ch == '\n' {
                starts.push(i + 1);
            }
        }
        Self { starts }
    }

    pub fn position(&self, offset: usize) -> Position {
        let line = self.starts.partition_point(|&s| s <= offset).saturating_sub(1);
        Position::new(line + 1, offset - self.starts[line] + 1)
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
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
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

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.position)
    }

    fn emit(&self, token_type: TokenType, start: usize) -> Token {
        Token::new(token_type, self.span_from(start))
    }

    fn take(&mut self, token_type: TokenType) -> Token {
        let start = self.position;
        self.advance();
        self.emit(token_type, start)
    }
    
    fn read_number(&mut self) -> Result<Token, TokenizeError> {
        let start = self.position;
        let loc = self.loc;

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
        let span = self.span_from(start);
        if extra_dot {
            return Err(TokenizeError::InvalidNumber(number_str, span, loc));
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(self.emit(TokenType::Number(number), start)),
            Err(_) => Err(TokenizeError::InvalidNumber(number_str, span, loc)),
        }
    }
    
    fn read_string(&mut self) -> Token {
        let start = self.position;
        self.advance(); // Skip opening quote
        let content_start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                break;
            }
            // TODO: Handle escape sequences
            self.advance();
        }
        
        let string_content: String = self.input[content_start..self.position].iter().collect();
        
        if self.current_char == Some('"') {
            self.advance(); // Skip closing quote
        }
        
        self.emit(TokenType::String(string_content), start)
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
        self.emit(TokenType::Symbol(symbol), start)
    }
    
    fn read_comment(&mut self) -> Token {
        let start = self.position;
        self.advance(); // Skip semicolon
        let content_start = self.position;
        
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
        
        let comment: String = self.input[content_start..self.position].iter().collect();
        self.emit(TokenType::Comment(comment), start)
    }
    
    pub fn next_token(&mut self) -> Result<Token, TokenizeError> {
        loop {
            match self.current_char {
                None => return Ok(Token::new(TokenType::Eof, Span::point(self.position))),
                
                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                
                Some('(') => return Ok(self.take(TokenType::LeftParen)),
                Some(')') => return Ok(self.take(TokenType::RightParen)),
                Some('\'') => return Ok(self.take(TokenType::Quote)),
                
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
                    let unknown_char = self.current_char.unwrap();
                    return Ok(self.take(TokenType::Unknown(unknown_char.to_string())));
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenizeError {
    Unknown(Token, Position),
    InvalidNumber(String, Span, Position),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = tokenizer.next_token()?;
        let is_eof = matches!(token.token_type, TokenType::Eof);

        if matches!(token.token_type, TokenType::Unknown(_)) {
            let loc = LineIndex::new(input).position(token.span.start);
            return Err(TokenizeError::Unknown(token, loc));
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
            TokenizeError::Unknown(token, loc) => {
                write!(
                    f,
                    "Unknown character at line {}, column {}: {:?}",
                    loc.line, loc.column, token.token_type
                )
            }
            TokenizeError::InvalidNumber(lexeme, _, loc) => {
                write!(
                    f,
                    "Invalid number '{}' at line {}, column {}",
                    lexeme, loc.line, loc.column
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
            Err(TokenizeError::InvalidNumber(lexeme, _, _)) => lexeme,
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
    fn invalid_number_includes_span() {
        let err = tokenize("(+ 1.2.3)").unwrap_err();
        match err {
            TokenizeError::InvalidNumber(lexeme, span, loc) => {
                assert_eq!(lexeme, "1.2.3");
                assert_eq!(span, Span::new(3, 8));
                assert_eq!(loc, Position::new(1, 4));
            }
            other => panic!("expected InvalidNumber, got {other:?}"),
        }
    }

    #[test]
    fn tokens_cover_source_span() {
        let tokens = tokenize("3.14").unwrap();
        assert_eq!(tokens[0].span, Span::new(0, 4));
    }
}