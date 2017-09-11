use nom::hex_digit;
use numeric_literal::numeric_literal::NumericLiteral;
use std::str::FromStr;
use std::str::from_utf8;

named!(pub hex_literal<NumericLiteral>,
    do_parse!(
        tag!("0x") >>
        value: hex_digit >>
        (
            || -> NumericLiteral {
                let tmp = from_utf8(value).unwrap();
                NumericLiteral::Integer(i32::from_str_radix(&tmp, 16).unwrap())
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_hex_literal_success_0() {
        let res = nom_value!(hex_literal, "0x1".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(1));
    }

    #[test]
    fn test_hex_literal_success_1() {
        let res = nom_value!(hex_literal, "0x0AG".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(10));
    }

    #[test]
    fn test_hex_literal_error_0() {
        let res = hex_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_hex_literal_error_1() {
        let res = hex_literal("0xG".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::HexDigit);
    }
}