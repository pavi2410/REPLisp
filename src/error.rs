use crate::tokenizer::{LineIndex, Position, Span};

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String, Span),
    TypeError(String, Span),
    ArityError(String, Span),
    DivisionByZero(Span),
    InvalidFunction(String, Span),
}

impl EvalError {
    pub fn span(&self) -> Span {
        match self {
            EvalError::UndefinedSymbol(_, span) => *span,
            EvalError::TypeError(_, span) => *span,
            EvalError::ArityError(_, span) => *span,
            EvalError::DivisionByZero(span) => *span,
            EvalError::InvalidFunction(_, span) => *span,
        }
    }
    
    pub fn set_span(&mut self, new_span: Span) {
        match self {
            EvalError::UndefinedSymbol(_, span) => *span = new_span,
            EvalError::TypeError(_, span) => *span = new_span,
            EvalError::ArityError(_, span) => *span = new_span,
            EvalError::DivisionByZero(span) => *span = new_span,
            EvalError::InvalidFunction(_, span) => *span = new_span,
        }
    }
    
    pub fn with_stack(self, stack: &[StackFrame]) -> EvalErrorWithStack {
        EvalErrorWithStack {
            error: self,
            stack_trace: stack.to_vec(),
        }
    }

    pub fn display(&self, source: &str) -> String {
        let loc = LineIndex::new(source).position(self.span().start);
        self.format_at(loc)
    }

    fn format_at(&self, loc: Position) -> String {
        match self {
            EvalError::UndefinedSymbol(s, _) => {
                format!("Undefined symbol '{}' at line {}, column {}", s, loc.line, loc.column)
            }
            EvalError::TypeError(msg, _) => {
                format!("Type error at line {}, column {}: {}", loc.line, loc.column, msg)
            }
            EvalError::ArityError(msg, _) => {
                format!("Arity error at line {}, column {}: {}", loc.line, loc.column, msg)
            }
            EvalError::DivisionByZero(_) => {
                format!("Division by zero at line {}, column {}", loc.line, loc.column)
            }
            EvalError::InvalidFunction(msg, _) => {
                format!("Invalid function at line {}, column {}: {}", loc.line, loc.column, msg)
            }
        }
    }
}

#[derive(Debug)]
pub struct EvalErrorWithStack {
    pub error: EvalError,
    pub stack_trace: Vec<StackFrame>,
}

impl EvalErrorWithStack {
    pub fn display(&self, source: &str) -> String {
        let lines = LineIndex::new(source);
        let mut out = self.error.display(source);
        if !self.stack_trace.is_empty() {
            out.push_str("\n\nStack trace:\n");
            for (i, frame) in self.stack_trace.iter().enumerate() {
                let loc = lines.position(frame.span.start);
                out.push_str(&format!(
                    "  {}: {} (line {}, column {})\n",
                    i + 1, frame.function_name, loc.line, loc.column
                ));
            }
        }
        out
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_at(Position::start()))
    }
}

impl std::fmt::Display for EvalErrorWithStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        if !self.stack_trace.is_empty() {
            writeln!(f, "\n\nStack trace:")?;
            for (i, frame) in self.stack_trace.iter().enumerate() {
                writeln!(
                    f,
                    "  {}: {} (offset {})",
                    i + 1, frame.function_name, frame.span.start
                )?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for EvalError {}
