use create_table_stmt::create_table_stmt_type::CreateTableStatement;
use nom::alphanumeric;
use std::str;

named!(schema_name<String>,
    do_parse!(
        name: ws!(alphanumeric) >>
        ws!(tag!(".")) >>
        (
            str::from_utf8(name).unwrap().to_string()
        )
    )
);

named!(pub create_table_stmt<CreateTableStatement>,
    do_parse!(
        ws!(tag!("CREATE")) >>
        tmp: ws!(opt!(alt_complete!(tag!("TEMP") | tag!("TEMPORARY")))) >>
        ws!(tag!("TABLE")) >>
        if_not_exists: ws!(opt!(delimited!(tag!("IF"), tag!("NOT"), tag!("EXISTS")))) >>
        schema_name: opt!(schema_name) >>
        table_name: ws!(alphanumeric) >>
        (
            CreateTableStatement::new()
        )
    )
);