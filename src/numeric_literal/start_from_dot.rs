use numeric_literal::numeric_literal_type::NumericLiteral;
use std::str::FromStr;
use std::str::from_utf8;
use nom::{digit};

named!(pub start_from_dot<NumericLiteral>,
    do_parse!(
        tag!(".") >>
        value: digit >>
        (
            || -> NumericLiteral {
                let t = from_utf8(value).unwrap();
                let tmp = format!("0.{}", t);
                NumericLiteral::Float(f64::from_str(&tmp).unwrap())
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_start_from_dot_success_0() {
        let res = nom_value!(start_from_dot, ".0".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.0));
    }

    #[test]
    fn test_start_from_dot_success_1() {
        let res = nom_value!(start_from_dot, ".012".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.012));
    }

    #[test]
    fn test_start_from_dot_success_2() {
        let res = nom_value!(start_from_dot, ".012ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.012));
    }

    #[test]
    fn test_start_from_dot_success_3() {
        let res = nom_value!(start_from_dot, ".42ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.42));
    }

    #[test]
    fn test_start_from_dot_error_0() {
        let res = start_from_dot("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_start_from_dot_error_1() {
        let res = start_from_dot(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }

    #[test]
    fn test_start_from_dot_error_2() {
        let res = start_from_dot("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_start_from_dot_error_3() {
        let res = start_from_dot(".".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }
}