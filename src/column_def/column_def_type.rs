use type_name::type_name_type::TypeName;
use column_constraint::column_constraint_type::ColumnConstraint;

#[derive(Debug, PartialEq)]
pub struct ColumnDef {
    pub column_name: String,
    pub type_name: Option<TypeName>,
    pub column_constraint_list: Vec<ColumnConstraint>
}