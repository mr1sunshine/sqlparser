use numeric_literal::hex_literal::hex_literal;
use numeric_literal::int_literal::int_literal;
use numeric_literal::numeric_literal_type::NumericLiteral;

named!(pub numeric_literal<NumericLiteral>,
    alt_complete!(
        hex_literal | int_literal
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_numeric_literal_success_0() {
        let res = nom_value!(numeric_literal, "10.1E-10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.00000000101));
    }

    #[test]
    fn test_numeric_literal_success_1() {
        let res = nom_value!(numeric_literal, "10.1E10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_numeric_literal_success_2() {
        let res = nom_value!(numeric_literal, "10.1E+10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_numeric_literal_success_3() {
        let res = nom_value!(numeric_literal, "10.1E-10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.00000000101));
    }

    #[test]
    fn test_numeric_literal_success_4() {
        let res = nom_value!(numeric_literal, "10.1E+10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_numeric_literal_success_5() {
        let res = nom_value!(numeric_literal, "10.1E10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_numeric_literal_success_6() {
        let res = nom_value!(numeric_literal, "0x1".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(1));
    }

    #[test]
    fn test_numeric_literal_success_7() {
        let res = nom_value!(numeric_literal, "0x10".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(16));
    }

    #[test]
    fn test_numeric_literal_success_8() {
        let res = nom_value!(numeric_literal, "0xG".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(0));
    }


    #[test]
    fn test_numeric_literal_error_0() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_1() {
        let res = numeric_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_2() {
        let res = numeric_literal("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_numeric_literal_error_3() {
        let res = numeric_literal(".".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_numeric_literal_error_4() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_5() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}