#[macro_use] extern crate lazy_static;

use config::Config;

lazy_static! {
    pub static ref SETTINGS: Config = {
        let mut config = Config::default();
        // Merge default configuration file
        config.merge(config::File::with_name("SettingsDefault.toml"))
            .expect("Missing default config");
        // Merge user configuration file; ignore if it doesn't exist
        config.merge(config::File::with_name("Settings.toml")).ok();
        config
    };
}