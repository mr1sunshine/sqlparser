use literal_value::literal_value_type::LiteralValue;

named!(pub literal_value_current_date<LiteralValue>, 
    do_parse!(
        tag!("CURRENT_DATE") >>
        (
            LiteralValue::CurrentDate
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_current_date_success_0() {
        let res = nom_value!(literal_value_current_date, "CURRENT_DATE".as_bytes());
        assert_eq!(res, LiteralValue::CurrentDate);
    }

    #[test]
    fn test_literal_value_current_date_incomplete() {
        let res = literal_value_current_date("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(12));
    }

    #[test]
    fn test_literal_value_current_date_error_0() {
        let res = literal_value_current_date("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_literal_value_current_date_error_1() {
        let res = literal_value_current_date("CURRENT_TIME".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }
}