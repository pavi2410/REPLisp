use crate::tokenizer::Position;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub position: Position,
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String, Position),
    TypeError(String, Position),
    ArityError(String, Position),
    DivisionByZero(Position),
    InvalidFunction(String, Position),
}

impl EvalError {
    pub fn position(&self) -> &Position {
        match self {
            EvalError::UndefinedSymbol(_, pos) => pos,
            EvalError::TypeError(_, pos) => pos,
            EvalError::ArityError(_, pos) => pos,
            EvalError::DivisionByZero(pos) => pos,
            EvalError::InvalidFunction(_, pos) => pos,
        }
    }
    
    pub fn set_position(&mut self, new_pos: Position) {
        match self {
            EvalError::UndefinedSymbol(_, pos) => *pos = new_pos,
            EvalError::TypeError(_, pos) => *pos = new_pos,
            EvalError::ArityError(_, pos) => *pos = new_pos,
            EvalError::DivisionByZero(pos) => *pos = new_pos,
            EvalError::InvalidFunction(_, pos) => *pos = new_pos,
        }
    }
    
    pub fn with_stack(self, stack: &[StackFrame]) -> EvalErrorWithStack {
        EvalErrorWithStack {
            error: self,
            stack_trace: stack.to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct EvalErrorWithStack {
    pub error: EvalError,
    pub stack_trace: Vec<StackFrame>,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UndefinedSymbol(s, pos) => {
                write!(f, "Undefined symbol '{}' at line {}, column {}", s, pos.line, pos.column)
            }
            EvalError::TypeError(msg, pos) => {
                write!(f, "Type error at line {}, column {}: {}", pos.line, pos.column, msg)
            }
            EvalError::ArityError(msg, pos) => {
                write!(f, "Arity error at line {}, column {}: {}", pos.line, pos.column, msg)
            }
            EvalError::DivisionByZero(pos) => {
                write!(f, "Division by zero at line {}, column {}", pos.line, pos.column)
            }
            EvalError::InvalidFunction(msg, pos) => {
                write!(f, "Invalid function at line {}, column {}: {}", pos.line, pos.column, msg)
            }
        }
    }
}

impl std::fmt::Display for EvalErrorWithStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)?;
        
        if !self.stack_trace.is_empty() {
            writeln!(f, "\nStack trace:")?;
            for (i, frame) in self.stack_trace.iter().enumerate() {
                writeln!(f, "  {}: {} (line {}, column {})", 
                        i + 1, frame.function_name, frame.position.line, frame.position.column)?;
            }
        }
        
        Ok(())
    }
}

impl std::error::Error for EvalError {}