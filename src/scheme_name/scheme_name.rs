use std::str;

named!(pub scheme_name<String>,
    do_parse!(
        scheme_name: is_a!( "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_" ) >>
        (
            str::from_utf8(scheme_name).unwrap().to_string()
        )
    )
);