use std::iter::Peekable;
use std::str::Chars;

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
    Number(f64),
    String(String),
    Symbol(String),
    LeftParen,
    RightParen,
    Quote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenType, span: Span) -> Self {
        Self { kind, span }
    }
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            position: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek_at(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.position += 1;
        Some(ch)
    }

    fn starts_number(&self, ch: char) -> bool {
        match ch {
            c if c.is_ascii_digit() => true,
            '.' => matches!(self.peek_at(1), Some(c) if c.is_ascii_digit() || c == '.'),
            '-' => match self.peek_at(1) {
                Some(c) if c.is_ascii_digit() => true,
                Some('.') => matches!(self.peek_at(2), Some(c) if c.is_ascii_digit() || c == '.'),
                _ => false,
            },
            _ => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.advance();
        }
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.position)
    }

    fn emit(&self, kind: TokenType, start: usize) -> Token {
        Token::new(kind, self.span_from(start))
    }

    fn take(&mut self, kind: TokenType) -> Token {
        let start = self.position;
        self.advance();
        self.emit(kind, start)
    }

    fn read_number(&mut self) -> Result<Token, TokenizeError> {
        let start = self.position;
        let mut number_str = String::new();

        if self.peek() == Some('-') {
            number_str.push(self.advance().unwrap());
        }

        let mut seen_dot = false;
        let mut extra_dot = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number_str.push(self.advance().unwrap());
            } else if ch == '.' {
                if seen_dot {
                    extra_dot = true;
                }
                seen_dot = true;
                number_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let span = self.span_from(start);
        if extra_dot {
            return Err(TokenizeError::InvalidNumber(number_str, span));
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(self.emit(TokenType::Number(number), start)),
            Err(_) => Err(TokenizeError::InvalidNumber(number_str, span)),
        }
    }

    fn read_string(&mut self) -> Result<Token, TokenizeError> {
        let start = self.position;
        self.advance();
        let mut content = String::new();

        loop {
            match self.advance() {
                None => return Err(TokenizeError::UnterminatedString(self.span_from(start))),
                Some('"') => return Ok(self.emit(TokenType::String(content), start)),
                Some('\\') => match self.advance() {
                    None => return Err(TokenizeError::UnterminatedString(self.span_from(start))),
                    Some(ch) => content.push(match ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        other => other,
                    }),
                },
                Some(ch) => content.push(ch),
            }
        }
    }

    fn read_symbol(&mut self) -> Token {
        let start = self.position;
        let mut symbol = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || "+-*/%=<>!?_-".contains(ch) {
                symbol.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        self.emit(TokenType::Symbol(symbol), start)
    }

    fn skip_comment(&mut self) {
        self.advance();
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, TokenizeError> {
        loop {
            match self.peek() {
                None => return Ok(None),

                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }

                Some('(') => return Ok(Some(self.take(TokenType::LeftParen))),
                Some(')') => return Ok(Some(self.take(TokenType::RightParen))),
                Some('\'') => return Ok(Some(self.take(TokenType::Quote))),

                Some('"') => return self.read_string().map(Some),

                Some(';') => {
                    self.skip_comment();
                    continue;
                }

                Some(ch) if self.starts_number(ch) => return self.read_number().map(Some),

                Some(ch) if ch.is_alphanumeric() || "+-*/%=<>!?_-".contains(ch) => {
                    return Ok(Some(self.read_symbol()));
                }

                Some(ch) => {
                    let start = self.position;
                    self.advance();
                    return Err(TokenizeError::Unknown(ch, self.span_from(start)));
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenizeError {
    Unknown(char, Span),
    InvalidNumber(String, Span),
    UnterminatedString(Span),
}

impl TokenizeError {
    pub fn display(&self, source: &str) -> String {
        let lines = LineIndex::new(source);
        match self {
            TokenizeError::Unknown(ch, span) => {
                let loc = lines.position(span.start);
                format!(
                    "Unknown character at line {}, column {}: {:?}",
                    loc.line, loc.column, ch
                )
            }
            TokenizeError::InvalidNumber(lexeme, span) => {
                let loc = lines.position(span.start);
                format!(
                    "Invalid number '{}' at line {}, column {}",
                    lexeme, loc.line, loc.column
                )
            }
            TokenizeError::UnterminatedString(span) => {
                let loc = lines.position(span.start);
                format!(
                    "Unterminated string at line {}, column {}",
                    loc.line, loc.column
                )
            }
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    
    while let Some(token) = tokenizer.next_token()? {
        tokens.push(token);
    }
    
    Ok(tokens)
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::Unknown(ch, span) => {
                write!(f, "Unknown character at offset {}: {:?}", span.start, ch)
            }
            TokenizeError::InvalidNumber(lexeme, span) => {
                write!(f, "Invalid number '{}' at offset {}", lexeme, span.start)
            }
            TokenizeError::UnterminatedString(span) => {
                write!(f, "Unterminated string at offset {}", span.start)
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
            .filter_map(|t| match t.kind {
                TokenType::Number(n) => Some(n),
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
        assert_eq!(numbers("5."), vec![5.0]);
        assert_eq!(numbers("-5."), vec![-5.0]);
    }

    #[test]
    fn parses_leading_dot_numbers() {
        assert_eq!(numbers(".5"), vec![0.5]);
        assert_eq!(numbers(".0"), vec![0.0]);
        assert_eq!(numbers(".25"), vec![0.25]);
        assert_eq!(numbers(".123"), vec![0.123]);
        assert_eq!(numbers("-.5"), vec![-0.5]);
        assert_eq!(numbers("-.0"), vec![-0.0]);
        assert_eq!(numbers("-.25"), vec![-0.25]);
        assert_eq!(numbers("-.123"), vec![-0.123]);
    }

    #[test]
    fn leading_dot_numbers_in_lists() {
        let tokens = tokenize("(+ .5 -.5 .25)").unwrap();
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenType::LeftParen,
                TokenType::Symbol("+".into()),
                TokenType::Number(0.5),
                TokenType::Number(-0.5),
                TokenType::Number(0.25),
                TokenType::RightParen,
            ]
        );
    }

    #[test]
    fn leading_dot_number_spans() {
        assert_eq!(tokenize(".5").unwrap()[0].span, Span::new(0, 2));
        assert_eq!(tokenize("-.5").unwrap()[0].span, Span::new(0, 3));
        assert_eq!(tokenize("(.5)").unwrap()[1].span, Span::new(1, 3));
    }

    #[test]
    fn bare_dot_is_unknown() {
        match tokenize(".").unwrap_err() {
            TokenizeError::Unknown('.', span) => assert_eq!(span, Span::new(0, 1)),
            other => panic!("expected Unknown, got {other:?}"),
        }
        match tokenize("-.").unwrap_err() {
            TokenizeError::Unknown('.', span) => assert_eq!(span, Span::new(1, 2)),
            other => panic!("expected Unknown, got {other:?}"),
        }
    }

    #[test]
    fn rejects_multiple_dots() {
        assert_eq!(invalid_number("1.2.3"), "1.2.3");
        assert_eq!(invalid_number("-1.2.3"), "-1.2.3");
        assert_eq!(invalid_number("1..2"), "1..2");
        assert_eq!(invalid_number("1.2."), "1.2.");
        assert_eq!(invalid_number(".1.2"), ".1.2");
        assert_eq!(invalid_number("-.1.2"), "-.1.2");
        assert_eq!(invalid_number("..1"), "..1");
    }

    #[test]
    fn invalid_number_includes_span() {
        let err = tokenize("(+ 1.2.3)").unwrap_err();
        match err {
            TokenizeError::InvalidNumber(lexeme, span) => {
                assert_eq!(lexeme, "1.2.3");
                assert_eq!(span, Span::new(3, 8));
            }
            other => panic!("expected InvalidNumber, got {other:?}"),
        }
    }

    #[test]
    fn tokens_cover_source_span() {
        let tokens = tokenize("3.14").unwrap();
        assert_eq!(tokens[0].span, Span::new(0, 4));
    }

    fn string_value(input: &str) -> String {
        match tokenize(input).unwrap()[0].kind.clone() {
            TokenType::String(s) => s,
            other => panic!("expected String, got {other:?}"),
        }
    }

    #[test]
    fn parses_strings() {
        assert_eq!(string_value(r#""hello""#), "hello");
        assert_eq!(string_value(r#""""#), "");
        assert_eq!(tokenize(r#""hi""#).unwrap()[0].span, Span::new(0, 4));
    }

    #[test]
    fn parses_string_escapes() {
        assert_eq!(string_value(r#""a\nb""#), "a\nb");
        assert_eq!(string_value(r#""a\tb""#), "a\tb");
        assert_eq!(string_value(r#""a\rb""#), "a\rb");
        assert_eq!(string_value(r#""say \"hi\"""#), r#"say "hi""#);
        assert_eq!(string_value(r#""path\\file""#), r"path\file");
    }

    #[test]
    fn rejects_unterminated_string() {
        let err = tokenize(r#""foo"#).unwrap_err();
        match err {
            TokenizeError::UnterminatedString(span) => {
                assert_eq!(span, Span::new(0, 4));
            }
            other => panic!("expected UnterminatedString, got {other:?}"),
        }
        assert!(tokenize(r#""foo"#).unwrap_err().display(r#""foo"#).contains("line 1"));
    }

    #[test]
    fn rejects_unterminated_string_after_escape() {
        let err = tokenize(r#""foo\"#).unwrap_err();
        match err {
            TokenizeError::UnterminatedString(span) => {
                assert_eq!(span, Span::new(0, 5));
            }
            other => panic!("expected UnterminatedString, got {other:?}"),
        }
    }

    #[test]
    fn unterminated_string_includes_position() {
        let input = "(print \"hi";
        let err = tokenize(input).unwrap_err();
        match err {
            TokenizeError::UnterminatedString(span) => {
                assert_eq!(span, Span::new(7, 10));
                let loc = LineIndex::new(input).position(span.start);
                assert_eq!(loc, Position::new(1, 8));
            }
            other => panic!("expected UnterminatedString, got {other:?}"),
        }
    }

    #[test]
    fn skips_comments_and_whitespace() {
        let tokens = tokenize("  ; comment\n  1  ; more\n  2").unwrap();
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(kinds, vec![TokenType::Number(1.0), TokenType::Number(2.0)]);
    }
}