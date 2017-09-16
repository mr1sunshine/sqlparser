use signed_number::signed_number_type::SignedNumber;
use numeric_literal::numeric_literal::numeric_literal;
use numeric_literal::numeric_literal_type::NumericLiteral;
use std::str::from_utf8;

named!(pub signed_number<SignedNumber>,
    do_parse!(
        sign: opt!(alt_complete!(tag!("+") | tag!("-"))) >>
        value: numeric_literal >>
        (
            || -> SignedNumber {
                let tmp_sign = sign;
                let mut res = 1;
                match tmp_sign {
                    Some(x) => {
                        let tmp = from_utf8(x).unwrap();
                        if tmp == "-" {
                            res = -1;
                        }
                    }
                    _ => (),
                }

                match value {
                    NumericLiteral::Integer(i) => {
                        SignedNumber::Integer(res * i)
                    },

                    NumericLiteral::Float(i) => {
                        SignedNumber::Float(res as f64 * i)
                    }
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
    fn test_signed_number_success_0() {
        let res = nom_value!(signed_number, "10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(0.00000000101));
    }

    #[test]
    fn test_signed_number_success_1() {
        let res = nom_value!(signed_number, "10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_2() {
        let res = nom_value!(signed_number, "10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_3() {
        let res = nom_value!(signed_number, "10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(0.00000000101));
    }

    #[test]
    fn test_signed_number_success_4() {
        let res = nom_value!(signed_number, "10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_5() {
        let res = nom_value!(signed_number, "10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_6() {
        let res = nom_value!(signed_number, "0x1".as_bytes());
        assert_eq!(res, SignedNumber::Integer(1));
    }

    #[test]
    fn test_signed_number_success_7() {
        let res = nom_value!(signed_number, "0x10".as_bytes());
        assert_eq!(res, SignedNumber::Integer(16));
    }

    #[test]
    fn test_signed_number_success_8() {
        let res = nom_value!(signed_number, "0xG".as_bytes());
        assert_eq!(res, SignedNumber::Integer(0));
    }

    #[test]
    fn test_signed_number_success_9() {
        let res = nom_value!(signed_number, "+10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(0.00000000101));
    }

    #[test]
    fn test_signed_number_success_10() {
        let res = nom_value!(signed_number, "+10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_11() {
        let res = nom_value!(signed_number, "+10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_12() {
        let res = nom_value!(signed_number, "+10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(0.00000000101));
    }

    #[test]
    fn test_signed_number_success_13() {
        let res = nom_value!(signed_number, "+10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_14() {
        let res = nom_value!(signed_number, "+10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(101000000000.0));
    }

    #[test]
    fn test_signed_number_success_15() {
        let res = nom_value!(signed_number, "+0x1".as_bytes());
        assert_eq!(res, SignedNumber::Integer(1));
    }

    #[test]
    fn test_signed_number_success_16() {
        let res = nom_value!(signed_number, "+0x10".as_bytes());
        assert_eq!(res, SignedNumber::Integer(16));
    }

    #[test]
    fn test_signed_number_success_17() {
        let res = nom_value!(signed_number, "+0xG".as_bytes());
        assert_eq!(res, SignedNumber::Integer(0));
    }

    #[test]
    fn test_signed_number_success_18() {
        let res = nom_value!(signed_number, "-10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-0.00000000101));
    }

    #[test]
    fn test_signed_number_success_19() {
        let res = nom_value!(signed_number, "-10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-101000000000.0));
    }

    #[test]
    fn test_signed_number_success_20() {
        let res = nom_value!(signed_number, "-10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-101000000000.0));
    }

    #[test]
    fn test_signed_number_success_21() {
        let res = nom_value!(signed_number, "-10.1E-10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-0.00000000101));
    }

    #[test]
    fn test_signed_number_success_22() {
        let res = nom_value!(signed_number, "-10.1E+10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-101000000000.0));
    }

    #[test]
    fn test_signed_number_success_23() {
        let res = nom_value!(signed_number, "-10.1E10".as_bytes());
        assert_eq!(res, SignedNumber::Float(-101000000000.0));
    }

    #[test]
    fn test_signed_number_success_24() {
        let res = nom_value!(signed_number, "-0x1".as_bytes());
        assert_eq!(res, SignedNumber::Integer(-1));
    }

    #[test]
    fn test_signed_number_success_25() {
        let res = nom_value!(signed_number, "-0x10".as_bytes());
        assert_eq!(res, SignedNumber::Integer(-16));
    }

    #[test]
    fn test_signed_number_success_26() {
        let res = nom_value!(signed_number, "-0xG".as_bytes());
        assert_eq!(res, SignedNumber::Integer(0));
    }

    #[test]
    fn test_signed_number_error_0() {
        let res = signed_number("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_1() {
        let res = signed_number(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_2() {
        let res = signed_number("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_signed_number_error_3() {
        let res = signed_number(".".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_signed_number_error_4() {
        let res = signed_number("+a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_5() {
        let res = signed_number("+.a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_6() {
        let res = signed_number("+".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(2));
    }

    #[test]
    fn test_signed_number_error_7() {
        let res = signed_number("+.".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_signed_number_error_8() {
        let res = signed_number("-a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_9() {
        let res = signed_number("-.a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_signed_number_error_10() {
        let res = signed_number("-".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(2));
    }

    #[test]
    fn test_signed_number_error_11() {
        let res = signed_number("-.".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }
}