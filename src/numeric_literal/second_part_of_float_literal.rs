use nom::{digit};
use std::str::FromStr;
use std::str::from_utf8;
use numeric_literal::numeric_literal::NumericLiteral;

named!(pub second_part_of_float_literal<NumericLiteral>,
    do_parse!(
        alt!(tag!("E") | tag!("e")) >>
        sign: opt!(alt!(tag!("+") | tag!("-"))) >>
        value: digit >>
        (
            || -> NumericLiteral {
                let mut output: String = "1E".to_owned();
                match sign {
                    Some(s) => {
                        let tmp = from_utf8(s).unwrap();
                        output.push_str(tmp);
                    },
                    _ => (),
                }
                let tmp = from_utf8(value).unwrap();
                output.push_str(tmp);
                NumericLiteral::Float(f64::from_str(&output).unwrap()) 
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_second_part_of_float_literal_success_0() {
        let res = nom_value!(second_part_of_float_literal, "E10.a".as_bytes());
        assert_eq!(res, NumericLiteral::Float(10000000000.0));
    }

    #[test]
    fn test_second_part_of_float_literal_success_1() {
        let res = nom_value!(second_part_of_float_literal, "E+10.a".as_bytes());
        assert_eq!(res, NumericLiteral::Float(10000000000.0));
    }

    #[test]
    fn test_second_part_of_float_literal_success_2() {
        let res = nom_value!(second_part_of_float_literal, "E-10.a".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.0000000001));

    }

    #[test]
    fn test_second_part_of_float_literal_error_0() {
        let res = second_part_of_float_literal("42.a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_second_part_of_float_literal_error_1() {
        let res = second_part_of_float_literal("Ea".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }

    #[test]
    fn test_second_part_of_float_literal_error_2() {
        let res = second_part_of_float_literal("E+a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }

    #[test]
    fn test_second_part_of_float_literal_error_3() {
        let res = second_part_of_float_literal("E-a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }

}