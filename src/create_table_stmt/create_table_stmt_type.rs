use create_table_stmt::table_description_type::TableDescription;

#[derive(Debug, PartialEq)]
pub struct CreateTableStatement {
    pub is_temp: bool,
    pub if_not_exists: bool,
    pub schema_name: Option<String>,
    pub table_name: String,
    pub description: TableDescription
}

impl CreateTableStatement {
    pub fn new() -> Self {
        CreateTableStatement {
            is_temp: false,
            if_not_exists: false,
            schema_name: Option::None,
            table_name: String::new(),
            description: TableDescription::new(),
        }
    }
}