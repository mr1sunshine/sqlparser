use nom::hex_digit;

named!(pub hex_literal<&[u8]>,
    recognize!(
        complete!(
            pair!(
                tag!("0x"),
                hex_digit
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_hex_literal_success_0() {
        let res = hex_literal("0x1".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"0x1"[..]));
    }

    #[test]
    fn test_hex_literal_success_1() {
        let res = hex_literal("0x0AG".as_bytes()).unwrap();
        assert_eq!(res, (&b"G"[..], &b"0x0A"[..]));
    }

    #[test]
    fn test_hex_literal_error_0() {
        let res = hex_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_hex_literal_error_1() {
        let res = hex_literal("0xG".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::HexDigit);
    }
}