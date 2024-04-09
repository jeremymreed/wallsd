use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub wallpaper_collection: String,
    pub oncalendar_string: String,
}

impl Config {
    pub fn load_config() -> Config {
        tracing::info!("Loading config");
        let  config: Config = match confy::load("wallsd", Some("config")) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Unable to load config!");
            }
        };

        tracing::info!("Loaded config");

        Config {
            wallpaper_collection: config.wallpaper_collection,
            oncalendar_string: config.oncalendar_string,
        }
    }
}