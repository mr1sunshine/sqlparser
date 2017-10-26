use qualified_table_name::qualified_table_name_type::QualifiedTableName;
use qualified_table_name::qualified_table_name_type::QualifiedTableNameType;
use table_name::table_name::table_name;
use helpers::schema_name_dot;

use nom::alphanumeric;
use std::str;

named!(not_indexed<QualifiedTableNameType>, 
    do_parse!(
        ws!(tuple!(tag_no_case!("NOT"), tag_no_case!("INDEXED"))) >>
        (
            QualifiedTableNameType::NotIndexed
        )
    )
);

named!(indexed_by<QualifiedTableNameType>, 
    do_parse!(
        ws!(tuple!(tag_no_case!("INDEXED"), tag_no_case!("BY"))) >>
        named_index: alphanumeric >>
        (
            QualifiedTableNameType::IndexedBy(str::from_utf8(named_index).unwrap().to_string())
        )
    )
);

named!(pub qualified_table_name<QualifiedTableName>,
    do_parse!(
        schema_name: opt!(complete!(ws!(schema_name_dot))) >>
        table_name: table_name >>
        qualified_table_name_type: opt!(alt!(complete!(not_indexed) | complete!(indexed_by))) >>
        (
            || -> QualifiedTableName {
                let mut result = QualifiedTableName::new();
                result.schema_name = schema_name;
                result.table_name = table_name;
                
                let tmp = qualified_table_name_type;
                match tmp {
                    Some(x) => result.qualified_table_name_type = x,
                    None => result.qualified_table_name_type = QualifiedTableNameType::OnlyName
                }
                
                result
            }()
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualified_table_name_success_0() {
        let res = nom_value!(qualified_table_name, "test_name".as_bytes());
        assert_eq!(res, QualifiedTableName { 
                            schema_name: Option::None,
                            table_name: "test_name".to_string(),
                            qualified_table_name_type: QualifiedTableNameType::OnlyName
                        }
        );
    }

    #[test]
    fn qualified_table_name_success_1() {
        let res = nom_value!(qualified_table_name, "test_scheme.test_name".as_bytes());
        assert_eq!(res, QualifiedTableName { 
                            schema_name: Some("test_scheme".to_string()),
                            table_name: "test_name".to_string(),
                            qualified_table_name_type: QualifiedTableNameType::OnlyName
                        }
        );
    }

    #[test]
    fn qualified_table_name_success_2() {
        let res = nom_value!(qualified_table_name, "test_scheme.test_name NOT INDEXED".as_bytes());
        assert_eq!(res, QualifiedTableName { 
                            schema_name: Some("test_scheme".to_string()),
                            table_name: "test_name".to_string(),
                            qualified_table_name_type: QualifiedTableNameType::NotIndexed
                        }
        );
    }

    #[test]
    fn qualified_table_name_success_3() {
        let res = nom_value!(qualified_table_name, "test_scheme.test_name INDEXED BY testindex".as_bytes());
        assert_eq!(res, QualifiedTableName { 
                            schema_name: Some("test_scheme".to_string()),
                            table_name: "test_name".to_string(),
                            qualified_table_name_type: QualifiedTableNameType::IndexedBy("testindex".to_string())
                        }
        );
    }
}
