use literal_value::serialize::hex::{FromHex};
use literal_value::literal_value_type::LiteralValue;


named!(pub literal_value_blob<LiteralValue>, 
    do_parse!(
        value: delimited!(
            alt!(tag!("X'") | tag!("x'")),
            take_until!("'"),
            tag!("'")
        ) >>
        (
            || -> LiteralValue {
                let tmp_str = String::from_utf8(Vec::from(value)).unwrap();
                let output = tmp_str.from_hex().unwrap();
                LiteralValue::BlobLiteral(output.into_boxed_slice())
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_literal_value_blob_success_0() {
        let res = nom_value!(literal_value_blob, "x'11223344'".as_bytes());
        let test = Box::new([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_blob_success_1() {
        let res = nom_value!(literal_value_blob, "X'11223344'".as_bytes());
        let test = Box::new([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_blob_success_2() {
        let res = nom_value!(literal_value_blob, "X''".as_bytes());
        let test = Box::new([]);
        assert_eq!(res, LiteralValue::BlobLiteral(test));
    }

    #[test]
    fn test_literal_value_blob_incomplete_0() {
        let res = literal_value_blob("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(2));
    }

    #[test]
    fn test_literal_value_blob_incomplete_1() {
        let res = literal_value_blob("X".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(2));
    }
    
    #[test]
    fn test_literal_value_blob_incomplete_2() {
        let res = literal_value_blob("x'".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Size(3));
    }

    #[test]
    fn test_literal_value_blob_error_1() {
        let res = literal_value_blob("x'444".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::TakeUntil);
    }

    #[test]
    fn test_literal_value_blob_error_2() {
        let res = literal_value_blob("x'GGG".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::TakeUntil);
    }

    #[test]
    #[should_panic]
    fn test_literal_value_blob_error_3() {
        literal_value_blob("x'444'".as_bytes()).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn test_literal_value_blob_error_4() {
        literal_value_blob("x'GG'".as_bytes()).unwrap_err();
    }
}