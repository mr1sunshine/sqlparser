use literal_value::literal_value_type::LiteralValue;

named!(pub literal_value_current_timestamp<LiteralValue>, 
    do_parse!(
        tag_no_case!("CURRENT_TIMESTAMP") >>
        (
            LiteralValue::CurrentTimestamp
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_current_timestamp_success_0() {
        let res = nom_value!(literal_value_current_timestamp, "CURRENT_TIMESTAMP".as_bytes());
        assert_eq!(res, LiteralValue::CurrentTimestamp);
    }

    #[test]
    fn test_literal_value_current_timestamp_incomplete_0() {
        let res = literal_value_current_timestamp("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(17));
    }

    #[test]
    fn test_literal_value_current_timestamp_incomplete_1() {
        let res = literal_value_current_timestamp("CURRENT_TIME".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(17));
    }

    #[test]
    fn test_literal_value_current_timestamp_error_0() {
        let res = literal_value_current_timestamp("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_literal_value_current_timestamp_error_1() {
        let res = literal_value_current_timestamp("CURRENT_TIMEAAAAA".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }
}