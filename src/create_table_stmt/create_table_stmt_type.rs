use column_def::column_def_type::ColumnDef;
use table_constraint::table_constraint_type::TableConstraint;

#[derive(Debug, PartialEq)]
enum TableColumns {
    Described {
        column_definitions: Vec<ColumnDef>,
        table_constraints: Vec<TableConstraint>,
        without_rowid: bool
    },
    Selected
}

impl TableColumns {
    fn new() -> Self {
        TableColumns::Described {
            column_definitions: Vec::new(),
            table_constraints: Vec::new(),
            without_rowid: false
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateTableStatement {
    is_temp: bool,
    if_not_exists: bool,
    schema_name: Option<String>,
    table_name: String,
    columns: TableColumns
}

impl CreateTableStatement {
    pub fn new() -> Self {
        CreateTableStatement {
            is_temp: false,
            if_not_exists: false,
            schema_name: Option::None,
            table_name: String::new(),
            columns: TableColumns::new(),
        }
    }
}