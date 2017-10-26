#[macro_export]

#[allow(unused_macros)]
macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t).to_result())
}

#[allow(unused_macros)]
macro_rules! nom_value {
    ($p:expr,$t:expr) => ($p($t).to_result().unwrap())
}

use nom::alphanumeric;
use std::str;

named!(pub schema_name_dot<String>,
    do_parse!(
        name: alphanumeric >>
        tag!(".") >>
        (
            str::from_utf8(name).unwrap().to_string()
        )
    )
);