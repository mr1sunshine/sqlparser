use numeric_literal::digits::digits;
use numeric_literal::start_from_dot::start_from_dot;
use numeric_literal::numeric_literal_type::NumericLiteral;

named!(pub maybe_float_literal<NumericLiteral>,
    do_parse!(
        first: digits >>
        second: opt!(complete!(start_from_dot)) >>
        (
            || -> NumericLiteral {
                let tmp = second;
                match tmp {
                    Some(float_part) => first + float_part,
                    None => first
                }
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_maybe_float_literal_success_0() {
        let res = nom_value!(maybe_float_literal, "0".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(0));
    }

    #[test]
    fn test_maybe_float_literal_success_1() {
        let res = nom_value!(maybe_float_literal, "012".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_maybe_float_literal_success_2() {
        let res = nom_value!(maybe_float_literal, "012ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_maybe_float_literal_success_3() {
        let res = nom_value!(maybe_float_literal, "42ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_maybe_float_literal_success_4() {
        let res = nom_value!(maybe_float_literal, "42.0ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(42.0));
    }

    #[test]
    fn test_maybe_float_literal_success_5() {
        let res = nom_value!(maybe_float_literal, "42.0".as_bytes());
        assert_eq!(res, NumericLiteral::Float(42.0));
    }

    #[test]
    fn test_maybe_float_literal_success_6() {
        let res = nom_value!(maybe_float_literal, "42.".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_maybe_float_literal_success_7() {
        let res = nom_value!(maybe_float_literal, "42.a".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_maybe_float_literal_success_8() {
        let res = nom_value!(maybe_float_literal, "42".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

     #[test]
    fn test_maybe_float_literal_error_0() {
        let res = maybe_float_literal("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_maybe_float_literal_error_1() {
        let res = maybe_float_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }
}