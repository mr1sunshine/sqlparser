#[macro_export]

#[allow(unused_macros)]
macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t).to_result())
}

#[allow(unused_macros)]
macro_rules! nom_value {
    ($p:expr,$t:expr) => ($p($t).to_result().unwrap())
}

use scheme_name::scheme_name::scheme_name;

named!(pub schema_name_dot<String>,
    do_parse!(
        name: scheme_name >>
        tag!(".") >>
        (
            name
        )
    )
);