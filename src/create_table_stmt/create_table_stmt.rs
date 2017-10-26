use create_table_stmt::create_table_stmt_type::CreateTableStatement;
use create_table_stmt::table_description::table_description;
use table_name::table_name::table_name;
use helpers::schema_name_dot;

named!(pub create_table_stmt<CreateTableStatement>,
    do_parse!(
        ws!(tag_no_case!("CREATE")) >>
        tmp: opt!(ws!(alt_complete!(tag_no_case!("TEMPORARY") | tag_no_case!("TEMP")))) >>
        ws!(tag_no_case!("TABLE")) >>
        if_not_exists: opt!(complete!(ws!(tuple!(tag_no_case!("IF"), tag_no_case!("NOT"), tag_no_case!("EXISTS"))))) >>
        schema_name: opt!(ws!(schema_name_dot)) >>
        table_name: table_name >>
        desc: ws!(table_description) >>
        (
            || -> CreateTableStatement {
                let mut result = CreateTableStatement::new();
                match tmp {
                    Some(x) => result.is_temp = true,
                    None => result.is_temp = false
                }
                match if_not_exists {
                    Some(x) => result.if_not_exists = true,
                    None => result.if_not_exists = false
                }
                result.schema_name = schema_name;
                result.table_name = table_name;
                result.description = desc;
                result
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use create_table_stmt::table_description_type::TableDescription;
    use column_def::column_def_type::ColumnDef;
    use type_name::type_name_type::TypeName;
    use signed_number::signed_number_type::SignedNumber;

    #[test]
    fn create_table_stmt_test_success_0() {
        let res = nom_value!(create_table_stmt, "CREATE TABLE some.table (id INT, name VARCHAR)".as_bytes());
        let reference = CreateTableStatement { 
            is_temp: false, 
            if_not_exists: false, 
            schema_name: Some("some".to_string()), 
            table_name: "table".to_string(), 
            description: TableDescription::Described { 
                column_definitions: vec![
                    ColumnDef { 
                        column_name: "id".to_string(), 
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["INT".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![] 
                    }, 
                    ColumnDef { 
                        column_name: "name".to_string(),
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["VARCHAR".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![]
                    }
                ], 
                table_constraints: vec![], 
                without_rowid: false
            } 
        };
        assert_eq!(res, reference);
    }

    #[test]
    fn create_table_stmt_test_success_1() {
        let res = nom_value!(create_table_stmt, "CREATE TEMPORARY TABLE IF NOT EXISTS some.table (id INT, name VARCHAR) WITHOUT ROWID".as_bytes());
        let reference = CreateTableStatement { 
            is_temp: true, 
            if_not_exists: true, 
            schema_name: Some("some".to_string()), 
            table_name: "table".to_string(), 
            description: TableDescription::Described { 
                column_definitions: vec![
                    ColumnDef { 
                        column_name: "id".to_string(), 
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["INT".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![] 
                    }, 
                    ColumnDef { 
                        column_name: "name".to_string(),
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["VARCHAR".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![]
                    }
                ], 
                table_constraints: vec![], 
                without_rowid: true
            } 
        };
        assert_eq!(res, reference);
    }

    #[test]
    fn create_table_stmt_test_success_2() {
        let res = nom_value!(create_table_stmt, "CREATE TABLE some.table (id INT, name VARCHAR (0x12), data REAL (5,5.2132))".as_bytes());
        let reference = CreateTableStatement { 
            is_temp: false, 
            if_not_exists: false, 
            schema_name: Some("some".to_string()), 
            table_name: "table".to_string(), 
            description: TableDescription::Described { 
                column_definitions: vec![
                    ColumnDef { 
                        column_name: "id".to_string(), 
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["INT".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![] 
                    }, 
                    ColumnDef { 
                        column_name: "name".to_string(),
                        type_name: Some(
                            TypeName::NameWithNumber { 
                                name: vec!["VARCHAR".to_string()],
                                number: SignedNumber::Integer(18)
                            }
                        ), 
                        column_constraint_list: vec![]
                    },
                    ColumnDef { 
                        column_name: "data".to_string(),
                        type_name: Some(
                            TypeName::NameWithTwoNumbers { 
                                name: vec!["REAL".to_string()],
                                number1: SignedNumber::Integer(5),
                                number2: SignedNumber::Float(5.2132)
                            }
                        ), 
                        column_constraint_list: vec![]
                    }
                ], 
                table_constraints: vec![], 
                without_rowid: false
            } 
        };
        assert_eq!(res, reference);
    }

    #[test]
    fn create_table_stmt_test_success_4() {
        let res = nom_value!(create_table_stmt, "create TABLE some.table (id INT, name VARCHAR)".as_bytes());
        let reference = CreateTableStatement { 
            is_temp: false, 
            if_not_exists: false, 
            schema_name: Some("some".to_string()), 
            table_name: "table".to_string(), 
            description: TableDescription::Described { 
                column_definitions: vec![
                    ColumnDef { 
                        column_name: "id".to_string(), 
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["INT".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![] 
                    }, 
                    ColumnDef { 
                        column_name: "name".to_string(),
                        type_name: Some(
                            TypeName::OnlyName { 
                                name: vec!["VARCHAR".to_string()]
                            }
                        ), 
                        column_constraint_list: vec![]
                    }
                ], 
                table_constraints: vec![], 
                without_rowid: false
            } 
        };
        assert_eq!(res, reference);
    }
}