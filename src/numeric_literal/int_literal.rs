use numeric_literal::first_part_of_int_literal::first_part_of_int_literal;
use numeric_literal::second_part_of_float_literal::second_part_of_float_literal;
use numeric_literal::numeric_literal::NumericLiteral;

named!(pub int_literal<NumericLiteral>,
    do_parse!(
        first: first_part_of_int_literal >>
        second: opt!(complete!(second_part_of_float_literal)) >>
        (
            || -> NumericLiteral {
                let tmp = second;
                match tmp {
                    Some(x) => {
                        first * x
                    }
                    None => first,
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
    fn test_int_literal_success_0() {
        let res = nom_value!(int_literal, "10.1E-10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.00000000101));
    }

    #[test]
    fn test_int_literal_success_1() {
        let res = nom_value!(int_literal, "10.1E10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_int_literal_success_2() {
        let res = nom_value!(int_literal, "10.1E+10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(101000000000.0));
    }

    #[test]
    fn test_int_literal_success_3() {
        let res = nom_value!(int_literal, "10E-10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.000000001));
    }

    #[test]
    fn test_int_literal_success_4() {
        let res = nom_value!(int_literal, "10E10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(100000000000.0));
    }

    #[test]
    fn test_int_literal_success_5() {
        let res = nom_value!(int_literal, "10E+10".as_bytes());
        assert_eq!(res, NumericLiteral::Float(100000000000.0));
    }

    #[test]
    fn test_int_literal_success_6() {
        let res = nom_value!(int_literal, "10".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(10));
    }

    #[test]
    fn test_int_literal_success_7() {
        let res = nom_value!(int_literal, "10.5".as_bytes());
        assert_eq!(res, NumericLiteral::Float(10.5));
    }

    #[test]
    fn test_int_literal_error_0() {
        let res = int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_1() {
        let res = int_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_2() {
        let res = int_literal("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_int_literal_error_3() {
        let res = int_literal(".".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_int_literal_error_4() {
        let res = int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}