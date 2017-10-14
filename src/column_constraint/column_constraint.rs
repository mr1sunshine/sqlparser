use column_constraint::column_constraint_type::ColumnConstraint;

named!(pub column_constraint<ColumnConstraint>,
    do_parse!(
        (
            ColumnConstraint::SomeValue
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_constraint() {
        let res = nom_value!(column_constraint, "test".as_bytes());
        assert_eq!(res, ColumnConstraint::SomeValue);
    }
}