use column_def::column_def_type::ColumnDef;
use type_name::type_name::type_name;
use column_constraint::column_constraint::column_constraint;
use nom::{alphanumeric};
use std::str;

named!(pub column_def<ColumnDef>,
    do_parse!(
        column_name: ws!(alphanumeric) >>
        type_name: opt!(complete!(type_name)) >>
        column_constraints: ws!(many0!(column_constraint)) >>
        (
            ColumnDef {
                column_name: str::from_utf8(column_name).unwrap().to_string(),
                type_name: type_name,
                column_constraint_list: column_constraints
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use type_name::type_name_type::TypeName;
    use signed_number::signed_number_type::SignedNumber;

    #[test]
    fn column_def_success_0() {
        let res = nom_value!(column_def, "test".as_bytes());
        assert_eq!(res, ColumnDef {
            column_name: "test".to_string(),
            type_name: None,
            column_constraint_list: vec![]
        });
    }

    #[test]
    fn column_def_success_1() {
        let res = nom_value!(column_def, "test int".as_bytes());
        assert_eq!(res, ColumnDef {
            column_name: "test".to_string(),
            type_name: Some(TypeName::OnlyName {
                name: vec!["int".to_string()]
            }),
            column_constraint_list: vec![]
        });
    }

    #[test]
    fn column_def_success_2() {
        let res = nom_value!(column_def, "test int test int".as_bytes());
        assert_eq!(res, ColumnDef {
            column_name: "test".to_string(),
            type_name: Some(TypeName::OnlyName {
                name: vec!["int".to_string(), "test".to_string(), "int".to_string()]
            }),
            column_constraint_list: vec![]
        });
    }

    #[test]
    fn column_def_success_3() {
        let res = nom_value!(column_def, "test int test int(5)".as_bytes());
        assert_eq!(res, ColumnDef {
            column_name: "test".to_string(),
            type_name: Some(TypeName::NameWithNumber {
                name: vec!["int".to_string(), "test".to_string(), "int".to_string()],
                number: SignedNumber::Integer(5)
            }),
            column_constraint_list: vec![]
        });
    }

    #[test]
    fn column_def_success_4() {
        let res = nom_value!(column_def, "test int test int(5,42)".as_bytes());
        assert_eq!(res, ColumnDef {
            column_name: "test".to_string(),
            type_name: Some(TypeName::NameWithTwoNumbers {
                name: vec!["int".to_string(), "test".to_string(), "int".to_string()],
                number1: SignedNumber::Integer(5),
                number2: SignedNumber::Integer(42)
            }),
            column_constraint_list: vec![]
        });
    }

}