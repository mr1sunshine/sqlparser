use literal_value::literal_value_type::LiteralValue;

named!(pub literal_value_null<LiteralValue>, 
    do_parse!(
        tag!("NULL") >>
        (
            LiteralValue::Null
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_null_success_0() {
        let res = nom_value!(literal_value_null, "NULL".as_bytes());
        assert_eq!(res, LiteralValue::Null);
    }

    #[test]
    fn test_literal_value_null_incomplete() {
        let res = literal_value_null("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(4));
    }

    #[test]
    fn test_literal_value_null_error() {
        let res = literal_value_null("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }
}