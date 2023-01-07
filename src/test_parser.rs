#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::{ReplispParser, Rule};

    #[test]
    fn it_should_parse_integers() {
        assert!(ReplispParser::parse(Rule::integer, "0").is_ok());
        assert!(ReplispParser::parse(Rule::integer, "1").is_ok());
        assert!(ReplispParser::parse(Rule::integer, "1234").is_ok());
        assert!(ReplispParser::parse(Rule::integer, "+100").is_ok());
        assert!(ReplispParser::parse(Rule::integer, "-100").is_ok());
    }

    #[test]
    fn it_should_parse_decimals() {
        assert!(ReplispParser::parse(Rule::decimal, "1.5").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "-1.5").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "+1.5").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "-01.5").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "-01.50").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, ".50").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "-.50").is_ok());
        assert!(ReplispParser::parse(Rule::decimal, "+.50").is_ok());
    }

    #[test]
    fn it_should_parse_booleans() {
        assert!(ReplispParser::parse(Rule::boolean, "true").is_ok());
        assert!(ReplispParser::parse(Rule::boolean, "false").is_ok());
    }

    #[test]
    fn it_should_parse_strings() {
        assert!(ReplispParser::parse(Rule::string, r#""""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#""Replisp""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#""🫡""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#""C:\Windows\Program Files""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#""\\""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r##""https://pavi2410.me/?q=query#fragment""##).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#""hello\nworld""#).is_ok());
        assert!(ReplispParser::parse(Rule::string, r#"""qouted text"""#).is_ok());
    }

    #[test]
    fn it_should_parse_idents() {
        assert!(ReplispParser::parse(Rule::ident, "a").is_ok());
        assert!(ReplispParser::parse(Rule::ident, "apple").is_ok());
        assert!(ReplispParser::parse(Rule::ident, "apple-juice").is_ok());
        assert!(ReplispParser::parse(Rule::ident, "apple_juice").is_ok());
        assert!(ReplispParser::parse(Rule::ident, "apple_juice_123").is_ok());
        assert!(ReplispParser::parse(Rule::ident, "BananaPie").is_ok());

        assert!(ReplispParser::parse(Rule::ident, "1var").is_err());
    }
}
