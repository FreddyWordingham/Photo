mod settings;

pub use settings::Settings;

pub fn run(settings: &Settings) {
    debug_assert!(settings.are_valid());

    println!("Hello, world!");

    println!("Settings: {:?}", settings);
}
