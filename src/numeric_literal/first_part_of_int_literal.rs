use numeric_literal::maybe_float_literal::maybe_float_literal;
use numeric_literal::start_from_dot::start_from_dot;
use numeric_literal::numeric_literal_type::NumericLiteral;

named!(pub first_part_of_int_literal<NumericLiteral>,
    alt_complete!(maybe_float_literal | start_from_dot)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_first_part_of_int_literal_success_0() {
        let res = nom_value!(first_part_of_int_literal, ".0".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.0));
    }

    #[test]
    fn test_first_part_of_int_literal_success_1() {
        let res = nom_value!(first_part_of_int_literal, ".012".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.012));
    }

    #[test]
    fn test_first_part_of_int_literal_success_2() {
        let res = nom_value!(first_part_of_int_literal, ".012ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.012));
    }

    #[test]
    fn test_first_part_of_int_literal_success_3() {
        let res = nom_value!(first_part_of_int_literal, ".42ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(0.42));
    }

    #[test]
    fn test_first_part_of_int_literal_success_4() {
        let res = nom_value!(first_part_of_int_literal, "0".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(0));
    }

    #[test]
    fn test_first_part_of_int_literal_success_5() {
        let res = nom_value!(first_part_of_int_literal, "012".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_first_part_of_int_literal_success_6() {
        let res = nom_value!(first_part_of_int_literal, "012ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(12));
    }

    #[test]
    fn test_first_part_of_int_literal_success_7() {
        let res = nom_value!(first_part_of_int_literal, "42ab".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_first_part_of_int_literal_success_8() {
        let res = nom_value!(first_part_of_int_literal, "42.0ab".as_bytes());
        assert_eq!(res, NumericLiteral::Float(42.0));
    }

    #[test]
    fn test_first_part_of_int_literal_success_9() {
        let res = nom_value!(first_part_of_int_literal, "42.0".as_bytes());
        assert_eq!(res, NumericLiteral::Float(42.0));
    }

    #[test]
    fn test_first_part_of_int_literal_success_10() {
        let res = nom_value!(first_part_of_int_literal, "42.".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_first_part_of_int_literal_success_11() {
        let res = nom_value!(first_part_of_int_literal, "42.a".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_first_part_of_int_literal_success_12() {
        let res = nom_value!(first_part_of_int_literal, "42".as_bytes());
        assert_eq!(res, NumericLiteral::Integer(42));
    }

    #[test]
    fn test_first_part_of_int_literal_error_0() {
        let res = first_part_of_int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_1() {
        let res = first_part_of_int_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_2() {
        let res = first_part_of_int_literal("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_first_part_of_int_literal_error_3() {
        let res = first_part_of_int_literal(".".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_first_part_of_int_literal_error_4() {
        let res = first_part_of_int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}