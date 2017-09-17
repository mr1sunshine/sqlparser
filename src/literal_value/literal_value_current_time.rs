use literal_value::literal_value_type::LiteralValue;

named!(pub literal_value_current_time<LiteralValue>, 
    do_parse!(
        tag!("CURRENT_TIME") >>
        (
            LiteralValue::CurrentTime
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_current_time_success_0() {
        let res = nom_value!(literal_value_current_time, "CURRENT_TIME".as_bytes());
        assert_eq!(res, LiteralValue::CurrentTime);
    }

    #[test]
    fn test_literal_value_current_time_incomplete() {
        let res = literal_value_current_time("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(12));
    }

    #[test]
    fn test_literal_value_current_time_error() {
        let res = literal_value_current_time("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }
}