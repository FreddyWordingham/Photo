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

/// Initialize the logger.
/// If the target is wasm32, use console_log.
/// Otherwise, use env_logger.
pub fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            log::info!("WASM logger initialized.");
        } else {
            env_logger::init();
            log::info!("Standard logger initialized.");
        }
    }
}

/// Parse a string of the form "WxH" into a tuple of (width, height).
pub fn parse_resolution_string<T: FromStr>(resolution: &str) -> (T, T)
where
    <T as FromStr>::Err: Debug,
{
    let mut split = resolution.split('x');
    let width = split.next().unwrap().parse::<T>().unwrap();
    let height = split.next().unwrap().parse::<T>().unwrap();
    (width, height)
}
