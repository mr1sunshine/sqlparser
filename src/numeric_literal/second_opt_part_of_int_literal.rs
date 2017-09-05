use numeric_literal::digits::digits;

named!(pub second_opt_part_of_int_literal<&[u8]>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_opt_part_of_int_literal_success_0() {
        let res = second_opt_part_of_int_literal("42.a".as_bytes()).unwrap();
        assert_eq!(res, (&b"42.a"[..], &b""[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_1() {
        let res = second_opt_part_of_int_literal("E1142.a".as_bytes()).unwrap();
        assert_eq!(res, (&b".a"[..], &b"E1142"[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_2() {
        let res = second_opt_part_of_int_literal("E+1142.a".as_bytes()).unwrap();
        assert_eq!(res, (&b".a"[..], &b"E+1142"[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_3() {
        let res = second_opt_part_of_int_literal("E-1142.a".as_bytes()).unwrap();
        assert_eq!(res, (&b".a"[..], &b"E-1142"[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_4() {
        let res = second_opt_part_of_int_literal("Ea".as_bytes()).unwrap();
        assert_eq!(res, (&b"Ea"[..], &b""[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_5() {
        let res = second_opt_part_of_int_literal("E+a".as_bytes()).unwrap();
        assert_eq!(res, (&b"E+a"[..], &b""[..]));
    }

    #[test]
    fn test_second_opt_part_of_int_literal_success_6() {
        let res = second_opt_part_of_int_literal("E-a".as_bytes()).unwrap();
        assert_eq!(res, (&b"E-a"[..], &b""[..]));
    }
}