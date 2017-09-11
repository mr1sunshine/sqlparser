#[macro_export]

#[allow(unused_macros)]
macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t).to_result())
}

#[allow(unused_macros)]
macro_rules! nom_value {
    ($p:expr,$t:expr) => ($p($t).to_result().unwrap())
}