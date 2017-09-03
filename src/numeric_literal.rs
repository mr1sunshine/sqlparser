use nom::{digit, hex_digit};

named!(digits<&[u8]>,
    recognize!(digit)
);

named!(start_from_dot<&[u8]>,
    recognize!(
        complete!(
            pair!(
                tag!("."),
                digits
            )
        )
    )
);

named!(maybe_float_literal<&[u8]>,
    recognize!(
        complete!(
            pair!(
                digits,
                opt!(start_from_dot)
            )
        )
    )
);

named!(first_part_of_int_literal<&[u8]>,
    alt!(
        maybe_float_literal |
        start_from_dot
    )
);

named!(second_opt_part_of_int_literal<&[u8]>,
    recognize!(
        opt!(
            complete!(
                tuple!(
                    tag!("E"),
                    opt!(
                        alt!(
                            tag!("+") | tag!("-")
                        )
                    ),
                    digits
                )
            )
        )
    )
);

named!(int_literal<&[u8]>,
    recognize!(
        complete!(
            pair!(
                first_part_of_int_literal,
                second_opt_part_of_int_literal
            )
        )
    )
);

named!(hex_literal<&[u8]>,
    recognize!(
        complete!(
            pair!(
                tag!("0x"),
                hex_digit
            )
        )
    )
);

named!(numeric_literal<&[u8]>,
    alt!(
        int_literal | hex_literal
    )
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

    #[test]
    fn test_maybe_float_literal_success_0() {
        let res = maybe_float_literal("0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"0"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_1() {
        let res = maybe_float_literal("012".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"012"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_2() {
        let res = maybe_float_literal("012ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"012"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_3() {
        let res = maybe_float_literal("42ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"42"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_4() {
        let res = maybe_float_literal("42.0ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"42.0"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_5() {
        let res = maybe_float_literal("42.0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"42.0"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_6() {
        let res = maybe_float_literal("42.".as_bytes()).unwrap();
        assert_eq!(res, (&b"."[..], &b"42"[..]));
    }

    #[test]
    fn test_maybe_float_literal_success_7() {
        let res = maybe_float_literal("42.a".as_bytes()).unwrap();
        assert_eq!(res, (&b".a"[..], &b"42"[..]));
    }

    #[test]
    fn test_maybe_float_literal_error_0() {
        let res = maybe_float_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Complete);
    }

    #[test]
    fn test_maybe_float_literal_error_1() {
        let res = maybe_float_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Digit);
    }
}