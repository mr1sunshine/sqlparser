use numeric_literal::digits::digits;

named!(pub start_from_dot<&[u8]>,
    recognize!(
        complete!(
            pair!(
                tag!("."),
                digits
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_start_from_dot_success_0() {
        let res = start_from_dot(".0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b".0"[..]));
    }

    #[test]
    fn test_start_from_dot_success_1() {
        let res = start_from_dot(".012".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b".012"[..]));
    }

    #[test]
    fn test_start_from_dot_success_2() {
        let res = start_from_dot(".012ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b".012"[..]));
    }

    #[test]
    fn test_start_from_dot_success_3() {
        let res = start_from_dot(".42ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b".42"[..]));
    }

    #[test]
    fn test_start_from_dot_error_0() {
        let res = start_from_dot("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Tag);
    }

    #[test]
    fn test_start_from_dot_error_1() {
        let res = start_from_dot(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }

    #[test]
    fn test_start_from_dot_error_2() {
        let res = start_from_dot("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Complete);
    }

    #[test]
    fn test_start_from_dot_error_3() {
        let res = start_from_dot(".".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Complete);
    }
}