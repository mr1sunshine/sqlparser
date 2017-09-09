use nom::{digit};
use std::str::FromStr;
use std::str::from_utf8;
use numeric_literal::numeric_literal::NumericLiteral;

named!(pub digits<NumericLiteral>, 
    do_parse!(
        value: digit >>
        (
            NumericLiteral::Integer(
                ||-> i32 {
                        let t = from_utf8(value).unwrap();
                        i32::from_str(t).unwrap()
                }()
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_digits_success_0() {
        let res = nom_value!(digits, "0".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(0));
    }

    #[test]
    fn test_digits_success_1() {
        let res = nom_value!(digits, "012".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_digits_success_2() {
        let res = nom_value!(digits, "012ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_digits_success_3() {
        let res = nom_value!(digits, "42ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_digits_incomplete() {
        let res = digits("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_digits_error() {
        let res = digits("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }
}