use std::{fmt::Debug, str::FromStr};

#[macro_export]
macro_rules! print_info {
    ($name:expr, $value:expr) => {
        info!("{:<30} : {}", $name, $value);
    };
    ($name:expr, $value:expr, $unit:expr) => {
        info!("{:<30} : {} {}", $name, $value, $unit);
    };
}

/// Parse a string of the form "WxH" into a tuple of (width, height).
#[allow(dead_code)]
pub fn parse_resolution_string<T: FromStr>(resolution: &str) -> (T, T)
where
    <T as FromStr>::Err: Debug,
{
    let mut split = resolution.split('x');
    let width = split.next().unwrap().parse::<T>().unwrap();
    let height = split.next().unwrap().parse::<T>().unwrap();
    (width, height)
}
