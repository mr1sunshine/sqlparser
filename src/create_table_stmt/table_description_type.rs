use column_def::column_def_type::ColumnDef;
use table_constraint::table_constraint_type::TableConstraint;

#[derive(Debug, PartialEq)]
pub enum TableDescription {
    Described {
        column_definitions: Vec<ColumnDef>,
        table_constraints: Vec<TableConstraint>,
        without_rowid: bool
    },
    Selected
}

impl TableDescription {
    pub fn new() -> Self {
        TableDescription::Described {
            column_definitions: Vec::new(),
            table_constraints: Vec::new(),
            without_rowid: false
        }
    }
}