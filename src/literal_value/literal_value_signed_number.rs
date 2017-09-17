use literal_value::literal_value_type::LiteralValue;
use signed_number::signed_number::signed_number;

named!(pub literal_value_signed_number<LiteralValue>, 
    do_parse!(
        value: signed_number >>
        (
            LiteralValue::SignedNumber(value)
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};
    use signed_number::signed_number_type::SignedNumber;

    #[test]
    fn test_literal_value_signed_number_success_0() {
        let res = nom_value!(literal_value_signed_number, "10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::SignedNumber(SignedNumber::Float(0.00000000101)));
    }

    #[test]
    fn test_literal_value_signed_number_success_1() {
        let res = nom_value!(literal_value_signed_number, "-10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::SignedNumber(SignedNumber::Float(-0.00000000101)));
    }

    #[test]
    fn test_literal_value_signed_number_success_2() {
        let res = nom_value!(literal_value_signed_number, "+42".as_bytes());
        assert_eq!(res, LiteralValue::SignedNumber(SignedNumber::Integer(42)));
    }

    #[test]
    fn test_literal_value_signed_number_success_3() {
        let res = nom_value!(literal_value_signed_number, "-42".as_bytes());
        assert_eq!(res, LiteralValue::SignedNumber(SignedNumber::Integer(-42)));
    }

    #[test]
    fn test_literal_value_signed_number_incomplete() {
        let res = literal_value_signed_number("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_literal_value_signed_number_error_0() {
        let res = literal_value_signed_number("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_signed_number_error_1() {
        let res = literal_value_signed_number("CURRENT_TIME".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}