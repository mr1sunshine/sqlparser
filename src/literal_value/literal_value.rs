use literal_value::literal_value_numeric_literal::literal_value_numeric_literal;
use literal_value::literal_value_string::literal_value_string;
use literal_value::literal_value_blob::literal_value_blob;
use literal_value::literal_value_null::literal_value_null;
use literal_value::literal_value_current_time::literal_value_current_time;
use literal_value::literal_value_current_date::literal_value_current_date;
use literal_value::literal_value_current_timestamp::literal_value_current_timestamp;
use literal_value::literal_value_type::LiteralValue;

named!(pub literal_value<LiteralValue>,
    alt_complete!(
        literal_value_numeric_literal |
        literal_value_string |
        literal_value_blob |
        literal_value_null |
        literal_value_current_timestamp |
        literal_value_current_time |
        literal_value_current_date
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};
    use numeric_literal::numeric_literal_type::NumericLiteral;

    #[test]
    fn test_literal_value_success_0() {
        let res = nom_value!(literal_value, "10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Float(0.00000000101)));
    }

    #[test]
    fn test_literal_value_success_1() {
        let res = nom_value!(literal_value, "10.1E-10".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Float(0.00000000101)));
    }

    #[test]
    fn test_literal_value_success_2() {
        let res = nom_value!(literal_value, "42".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Integer(42)));
    }

    #[test]
    fn test_literal_value_success_3() {
        let res = nom_value!(literal_value, "42".as_bytes());
        assert_eq!(res, LiteralValue::NumericLiteral(NumericLiteral::Integer(42)));
    }

    #[test]
    fn test_literal_value_success_4() {
        let res = nom_value!(literal_value, "'NULL'".as_bytes());
        assert_eq!(res, LiteralValue::StringLiteral("NULL".to_string()));
    }

    #[test]
    fn test_literal_value_success_5() {
        let res = nom_value!(literal_value, "'test string 42'".as_bytes());
        assert_eq!(res, LiteralValue::StringLiteral("test string 42".to_string()));
    }

    #[test]
    fn test_literal_value_success_6() {
        let res = nom_value!(literal_value, "x'11223344'".as_bytes());
        let test = Box::new([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_success_7() {
        let res = nom_value!(literal_value, "X'11223344'".as_bytes());
        let test = Box::new([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_success_8() {
        let res = nom_value!(literal_value, "X''".as_bytes());
        let test = Box::new([]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_success_9() {
        let res = nom_value!(literal_value, "NULL".as_bytes());
        assert_eq!(res, LiteralValue::Null);
    }

    #[test]
    fn test_literal_value_success_10() {
        let res = nom_value!(literal_value, "CURRENT_TIME".as_bytes());
        assert_eq!(res, LiteralValue::CurrentTime);
    }

    #[test]
    fn test_literal_value_success_11() {
        let res = nom_value!(literal_value, "CURRENT_DATE".as_bytes());
        assert_eq!(res, LiteralValue::CurrentDate);
    }

    #[test]
    fn test_literal_value_success_12() {
        let res = nom_value!(literal_value, "CURRENT_TIMESTAMP".as_bytes());
        assert_eq!(res, LiteralValue::CurrentTimestamp);
    }

    #[test]
    fn test_literal_value_incomplete_0() {
        let res = literal_value("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(12));
    }

    #[test]
    fn test_literal_value_error_0() {
        let res = literal_value("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_error_1() {
        let res = literal_value("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_error_2() {
        let res = literal_value("'TEST".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    #[should_panic]
    fn test_literal_value_error_3() {
        let arr: [u8; 7] = [0x27, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x27];
        literal_value(&arr).unwrap_err();
        //assert_eq!(res, ErrorKind::TakeUntil);
    }

    #[test]
    fn test_literal_value_error_4() {
        let res = literal_value("x'444".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_error_5() {
        let res = literal_value("x'GGG".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    #[should_panic]
    fn test_literal_value_error_6() {
        literal_value("x'444'".as_bytes()).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn test_literal_value_error_7() {
        literal_value("x'GG'".as_bytes()).unwrap_err();
    }

    #[test]
    fn test_literal_value_error_8() {
        let res = literal_value("X".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_literal_value_error_9() {
        let res = literal_value("x'".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

}