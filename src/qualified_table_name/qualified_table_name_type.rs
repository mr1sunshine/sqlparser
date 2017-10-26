#[derive(Debug, PartialEq)]
pub enum QualifiedTableNameType {
    OnlyName,
    IndexedBy(String),
    NotIndexed
}

#[derive(Debug, PartialEq)]
pub struct QualifiedTableName {
    pub schema_name: Option<String>,
    pub table_name: String,
    pub qualified_table_name_type: QualifiedTableNameType
}

impl QualifiedTableName {
    pub fn new() -> Self {
        QualifiedTableName {
            schema_name: Option::None,
            table_name: String::new(),
            qualified_table_name_type: QualifiedTableNameType::OnlyName,
        }
    }
}