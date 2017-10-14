use table_constraint::table_constraint_type::TableConstraint;

named!(pub table_constraint<TableConstraint>,
    do_parse!(
        (
            TableConstraint::SomeValue
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_constraint() {
        let res = nom_value!(table_constraint, "test".as_bytes());
        assert_eq!(res, TableConstraint::SomeValue);
    }
}