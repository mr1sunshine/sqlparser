use numeric_literal::hex_literal::hex_literal;
use numeric_literal::int_literal::int_literal;

named!(numeric_literal<&[u8]>,
    alt!(
        hex_literal | int_literal
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_numeric_literal_success_0() {
        let res = numeric_literal("10.1E-10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E-10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_1() {
        let res = numeric_literal("10.1E10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_2() {
        let res = numeric_literal("10.1E+10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E+10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_3() {
        let res = numeric_literal("10E-10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E-10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_4() {
        let res = numeric_literal("10E10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_5() {
        let res = numeric_literal("10E+10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E+10"[..]));
    }

    #[test]
    fn test_numeric_literal_success_6() {
        let res = numeric_literal("0x1".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"0x1"[..]));
    }

    #[test]
    fn test_numeric_literal_success_7() {
        let res = numeric_literal("0x0AG".as_bytes()).unwrap();
        assert_eq!(res, (&b"G"[..], &b"0x0A"[..]));
    }

    #[test]
    fn test_numeric_literal_success_8() {
        let res = numeric_literal("0xG".as_bytes()).unwrap();
        assert_eq!(res, (&b"xG"[..], &b"0"[..]));
    }


    #[test]
    fn test_numeric_literal_error_0() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_1() {
        let res = numeric_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_2() {
        let res = numeric_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_3() {
        let res = numeric_literal(".".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_4() {
        let res = numeric_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_5() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_numeric_literal_error_6() {
        let res = numeric_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}