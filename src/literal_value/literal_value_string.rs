use literal_value::literal_value_type::LiteralValue;

/*
 * @TODO Add check for incorrect UTF-8 sequence
 */
named!(pub literal_value_string<LiteralValue>, 
    do_parse!(
        value: delimited!(
            tag!("'"),
            take_until!("'"),
            tag!("'")
        ) >>
        (
            LiteralValue::StringLiteral(String::from_utf8(Vec::from(value)).unwrap())
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_string_success_0() {
        let res = nom_value!(literal_value_string, "'NULL'".as_bytes());
        assert_eq!(res, LiteralValue::StringLiteral("NULL".to_string()));
    }

    #[test]
    fn test_literal_value_string_success_1() {
        let res = nom_value!(literal_value_string, "'test string 42'".as_bytes());
        assert_eq!(res, LiteralValue::StringLiteral("test string 42".to_string()));
    }

    #[test]
    fn test_literal_value_string_incomplete() {
        let res = literal_value_string("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(1));
    }

    #[test]
    fn test_literal_value_string_error_0() {
        let res = literal_value_string("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_literal_value_string_error_1() {
        let res = literal_value_string("'TEST".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::TakeUntil);
    }

    #[test]
    #[should_panic]
    fn test_literal_value_string_error_2() {
        let arr: [u8; 7] = [0x27, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x27];
        let res = literal_value_string(&arr).unwrap_err();
        //assert_eq!(res, ErrorKind::TakeUntil);
    }

}