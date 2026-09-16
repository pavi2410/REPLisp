use crate::tokenizer::tokenize;
use crate::parser::parse;
use crate::evaluator::eval_expr_with_stack;
use crate::environment::Environment;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_number_in_parse_error() {
        let input = "(\n  +\n  1\n  2"; // Missing closing paren
        
        let tokens = tokenize(input).unwrap();
        let result = parse(tokens);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        println!("Parse error: {}", error.display(input));
        
        // Should show line number where the unmatched paren started
        assert!(error.display(input).contains("line 1"));
    }

    #[test]
    fn test_line_number_in_tokenizer_error() {
        let input = "(\n  +\n  1\n  @"; // Invalid character
        
        let result = tokenize(input);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.display(input).contains("line 4"));
    }

    #[test]
    fn test_line_number_in_eval_error() {
        let input = "(\n  def\n  x)"; // Wrong number of args to def
        
        let tokens = tokenize(input).unwrap();
        let expressions = parse(tokens).unwrap();
        let mut env = Environment::new();
        let mut stack = Vec::new();
        
        let result = eval_expr_with_stack(&expressions[0], &mut env, &mut stack);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_with_stack = error.with_stack(&stack);
        println!("Eval error: {}", error_with_stack.display(input));
        
        // Should show line number where the error occurred
        assert!(error_with_stack.display(input).contains("line"));
    }

    #[test]
    fn test_basic_arithmetic_with_line_numbers() {
        let input = "(+ 1 2)";
        
        let tokens = tokenize(input).unwrap();
        let expressions = parse(tokens).unwrap();
        let mut env = Environment::new();
        let mut stack = Vec::new();
        
        let result = eval_expr_with_stack(&expressions[0], &mut env, &mut stack);
        
        assert!(result.is_ok());
        println!("Result: {:?}", result.unwrap());
    }
}