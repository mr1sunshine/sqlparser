use literal_value::literal_value_type::LiteralValue;
use numeric_literal::numeric_literal::numeric_literal;

named!(pub literal_value_numeric_literal<LiteralValue>, 
    do_parse!(
        value: numeric_literal >>
        (
            LiteralValue::NumericLiteral(value)
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};
    use numeric_literal::numeric_literal_type::NumericLiteral;

    #[test]
    fn test_literal_value_numeric_literal_success_0() {
        let res = nom_value!(literal_value_numeric_literal, "10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Float(0.00000000101)));
    }

    #[test]
    fn test_literal_value_numeric_literal_success_1() {
        let res = nom_value!(literal_value_numeric_literal, "10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Float(0.00000000101)));
    }

    #[test]
    fn test_literal_value_numeric_literal_success_2() {
        let res = nom_value!(literal_value_numeric_literal, "42".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Integer(42)));
    }

    #[test]
    fn test_literal_value_numeric_literal_success_3() {
        let res = nom_value!(literal_value_numeric_literal, "42".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Integer(42)));
    }

    #[test]
    fn test_literal_value_numeric_literal_incomplete() {
        let res = literal_value_numeric_literal("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_literal_value_numeric_literal_error_0() {
        let res = literal_value_numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_numeric_literal_error_1() {
        let res = literal_value_numeric_literal("CURRENT_TIME".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}