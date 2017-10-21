use create_table_stmt::table_description_type::TableDescription;
use column_def::column_def::column_def;
use column_def::column_def_type::ColumnDef;
use table_constraint::table_constraint_type::TableConstraint;
use table_constraint::table_constraint::table_constraint;

named!(comma_column_def<ColumnDef>, 
    do_parse!(
        ws!(tag!(",")) >>
        value: ws!(column_def) >>
        (
            value
        )
    )
);

named!(column_def_list<Vec<ColumnDef>>,
    do_parse!(
        one: ws!(column_def) >>
        list: many0!(comma_column_def) >>
        (
            || -> Vec<ColumnDef> {
                let mut tmp = Vec::new();
                tmp.push(one);
                tmp.extend(list);

                tmp
            }()
        )
    )
);

named!(comma_table_constraint<TableConstraint>, 
    do_parse!(
        tag!(",") >>
        constraint: table_constraint >>
        (
            constraint
        )
    )
);

named!(pub table_description<TableDescription>,
    do_parse!(
        tag!("(") >>
        columns: column_def_list >>
        table_constraints: many0!(comma_table_constraint) >>
        tag!(")") >>
        without_rowid: opt!(complete!(ws!(tuple!(tag_no_case!("WITHOUT"), tag_no_case!("ROWID"))))) >>
        (
            || -> TableDescription {
                let tmp: bool;
                match without_rowid {
                    Some(x) => tmp = true,
                    None => tmp = false,
                }

                TableDescription::Described {
                    column_definitions: columns,
                    table_constraints: table_constraints,
                    without_rowid: tmp
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
    fn comma_column_def_test() {
        let res = nom_value!(comma_column_def, ", id INT".as_bytes());
        assert_eq!(res, ColumnDef { 
                            column_name: "id".to_string(),
                            type_name: Some(
                                TypeName::OnlyName { 
                                    name: vec!["INT".to_string()]
                                }
                            ), 
                            column_constraint_list: vec![]
                        }
        );
    }

    #[test]
    fn column_def_list_test() {
        let res = nom_value!(column_def_list, "id INT, name VARCHAR".as_bytes());
        let reference = vec![
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
        ];
        assert_eq!(res, reference);
    }
}