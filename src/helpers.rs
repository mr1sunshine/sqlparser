#[macro_export]

macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t).to_result())
}

macro_rules! nom_value {
    ($p:expr,$t:expr) => ($p($t).to_result().unwrap())
}