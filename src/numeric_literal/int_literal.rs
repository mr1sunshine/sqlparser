use numeric_literal::first_part_of_int_literal::first_part_of_int_literal;
use numeric_literal::second_opt_part_of_int_literal::second_opt_part_of_int_literal;

named!(pub int_literal<&[u8]>,
    recognize!(
        complete!(
            pair!(
                first_part_of_int_literal,
                second_opt_part_of_int_literal
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_int_literal_success_0() {
        let res = int_literal("10.1E-10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E-10"[..]));
    }

    #[test]
    fn test_int_literal_success_1() {
        let res = int_literal("10.1E10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E10"[..]));
    }

    #[test]
    fn test_int_literal_success_2() {
        let res = int_literal("10.1E+10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10.1E+10"[..]));
    }

    #[test]
    fn test_int_literal_success_3() {
        let res = int_literal("10E-10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E-10"[..]));
    }

    #[test]
    fn test_int_literal_success_4() {
        let res = int_literal("10E10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E10"[..]));
    }

    #[test]
    fn test_int_literal_success_5() {
        let res = int_literal("10E+10".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"10E+10"[..]));
    }

    #[test]
    fn test_int_literal_error_0() {
        let res = int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_1() {
        let res = int_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_2() {
        let res = int_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_3() {
        let res = int_literal(".".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_4() {
        let res = int_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_int_literal_error_5() {
        let res = int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}