use std::str;

named!(pub table_name<String>,
    do_parse!(
        table_name: is_a!( "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_" ) >>
        (
            str::from_utf8(table_name).unwrap().to_string()
        )
    )
);