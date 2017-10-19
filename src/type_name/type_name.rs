use type_name::type_name_type::TypeName;
use signed_number::signed_number::signed_number;
use signed_number::signed_number_type::SignedNumber;
use nom::{alphanumeric};

use std::str;

named!(one_number<SignedNumber>,
    do_parse!(
        tag!("(") >>
        number : signed_number >>
        tag!(")") >>
        (
            number
        )
    )
);

named!(two_numbers<(SignedNumber, SignedNumber)>,
    do_parse!(
        tag!("(") >>
        number1 : ws!(signed_number) >>
        tag!(",") >>
        number2 : ws!(signed_number) >>
        tag!(")") >>
        (
            (number1, number2)
        )
    )
);

named!(pub type_name<TypeName>,
    do_parse!(
        words: many1!(ws!(alphanumeric)) >>
        opt_one_number: opt!(complete!(ws!(one_number))) >>
        opt_two_numbers: opt!(complete!(ws!(two_numbers))) >>
        (
            || -> TypeName {
                let words_utf: Vec<String> = words.into_iter().map(|x| str::from_utf8(x).unwrap().to_string()).collect();
                let tmp_opt_one_number = opt_one_number;
                match tmp_opt_one_number {
                    Some(x) => {
                        return TypeName::NameWithNumber {
                            name: words_utf,
                            number: x
                        }
                    }
                    _ => (),
                }
                let tmp_opt_two_numbers = opt_two_numbers;
                match tmp_opt_two_numbers {
                    Some(x) => {
                        return TypeName::NameWithTwoNumbers {
                            name: words_utf,
                            number1: x.0,
                            number2: x.1
                        }
                    }
                    _ => (),
                }
                TypeName::OnlyName {
                    name: words_utf
                }
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use type_name::type_name_type::TypeName;

    #[test]
    fn type_name_success_0() {
        let res = nom_value!(type_name, "test".as_bytes());
        assert_eq!(res, TypeName::OnlyName{
            name: vec!["test".to_string()]
        });
    }

    #[test]
    fn type_name_success_1() {
        let res = nom_value!(type_name, "test42 test".as_bytes());
        assert_eq!(res, TypeName::OnlyName{
            name: vec!["test42".to_string(), "test".to_string()]
        });
    }

    #[test]
    fn type_name_success_2() {
        let res = nom_value!(type_name, "test test 333".as_bytes());
        assert_eq!(res, TypeName::OnlyName{
            name: vec!["test".to_string(), "test".to_string(), "333".to_string()]
        });
    }

    #[test]
    fn type_name_success_3() {
        let res = nom_value!(type_name, "    test    ".as_bytes());
        assert_eq!(res, TypeName::OnlyName{
            name: vec!["test".to_string()]
        });
    }

    #[test]
    fn type_name_success_4() {
        let res = nom_value!(type_name, "    test    test     ".as_bytes());
        assert_eq!(res, TypeName::OnlyName{
            name: vec!["test".to_string(), "test".to_string()]
        });
    }

    #[test]
    fn type_name_success_5() {
        let res = nom_value!(type_name, "    test    test     (5)".as_bytes());
        assert_eq!(res, TypeName::NameWithNumber {
            name: vec!["test".to_string(), "test".to_string()],
            number: SignedNumber::Integer(5)
        });
    }

    #[test]
    fn type_name_success_6() {
        let res = nom_value!(type_name, "    test    test     (5,5)".as_bytes());
        assert_eq!(res, TypeName::NameWithTwoNumbers {
            name: vec!["test".to_string(), "test".to_string()],
            number1: SignedNumber::Integer(5),
            number2: SignedNumber::Integer(5)
        });
    }

    #[test]
    fn type_name_success_7() {
        let res = nom_value!(type_name, "    test    test     (   5   ,   5   )".as_bytes());
        assert_eq!(res, TypeName::NameWithTwoNumbers {
            name: vec!["test".to_string(), "test".to_string()],
            number1: SignedNumber::Integer(5),
            number2: SignedNumber::Integer(5)
        });
    }

}