#[cfg(test)]
mod tests {
    use crate::parser::{lexer, parser, parse, Token};
    use crate::lval::Lval;
    use chumsky::prelude::*;

    #[test]
    fn test_lexer() {
        let tokens = lexer().parse("(+ 1 2)").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(format!("{}", tokens[0]), "(");
        assert_eq!(format!("{}", tokens[1]), "+");
        assert_eq!(format!("{}", tokens[2]), "1");
        assert_eq!(format!("{}", tokens[3]), "2");
        assert_eq!(format!("{}", tokens[4]), ")");
    }

    #[test]
    fn test_parse_function() {
        // Test the parse function directly
        let result = parse("(+ 1 2)").unwrap();
        
        if let Lval::Sexpr(children) = *result {
            assert_eq!(children.len(), 3);
            if let Lval::Sym(s) = *children[0] {
                assert_eq!(s, "+");
            } else {
                panic!("Expected symbol, got {:?}", children[0]);
            }
        } else {
            panic!("Expected sexpr, got {:?}", result);
        }
    }

    #[test]
    fn test_nested_expression() {
        // Test nested expressions
        let result = parse("(+ 1 (* 2 3))").unwrap();
        
        if let Lval::Sexpr(children) = *result {
            assert_eq!(children.len(), 3);
            
            // Check the nested expression
            if let Lval::Sexpr(nested) = *children[2] {
                assert_eq!(nested.len(), 3);
                if let Lval::Sym(s) = *nested[0] {
                    assert_eq!(s, "*");
                } else {
                    panic!("Expected symbol, got {:?}", nested[0]);
                }
            } else {
                panic!("Expected sexpr, got {:?}", children[2]);
            }
        } else {
            panic!("Expected sexpr, got {:?}", result);
        }
    }

    #[test]
    fn test_qexpr() {
        // Test Q-expressions
        let result = parse("{1 2 3}").unwrap();
        
        if let Lval::Qexpr(children) = *result {
            assert_eq!(children.len(), 3);
            
            for (i, child) in children.iter().enumerate() {
                if let Lval::Num(n) = **child {
                    assert_eq!(n, (i + 1) as i64);
                } else {
                    panic!("Expected number, got {:?}", child);
                }
            }
        } else {
            panic!("Expected qexpr, got {:?}", result);
        }
    }
}
