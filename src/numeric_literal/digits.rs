use nom::{digit};

named!(pub digits<&[u8]>,
    recognize!(digit)
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind, Needed};

    #[test]
    fn test_digits_success_0() {
        let res = digits("0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"0"[..]));
    }

    #[test]
    fn test_digits_success_1() {
        let res = digits("012".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"012"[..]));
    }

    #[test]
    fn test_digits_success_2() {
        let res = digits("012ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"012"[..]));
    }

    #[test]
    fn test_digits_success_3() {
        let res = digits("42ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"42"[..]));
    }

    #[test]
    fn test_digits_incomplete() {
        let res = digits("".as_bytes()).unwrap_inc();
        assert_eq!(res, Needed::Unknown);
    }

    #[test]
    fn test_digits_error() {
        let res = digits("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }
}