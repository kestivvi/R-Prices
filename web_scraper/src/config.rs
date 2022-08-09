use std::collections::HashMap;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PriceScraperConfig {
    pub user_agent: String,
    pub database_url: String,
    pub run_in_loop: bool,
    pub interval: u64,
    pub reqwest_selectors: HashMap<String, String>,
    pub fantoccini_selectors: HashMap<String, String>,
}

impl PriceScraperConfig {
    /// Exits the program if could not load web_scraper_config.json or there are values not set
    pub fn new() -> PriceScraperConfig {
        Self::new_from_file("web_scraper_settings")
    }

    pub fn new_from_file(path: &str) -> PriceScraperConfig {
        let settings = Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default())
            .build();

        let settings = match settings {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "Could not load the config for some reason: {}",
                    e.to_string()
                );
                // process::exit(1);
            }
        };

        match settings.try_deserialize::<PriceScraperConfig>() {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "Error parsing web_scraper_config.json to WebScraperConfig struct. Is web_scraper_config.json has all needed values? Raw error: {}", 
                    e
                );
                // process::exit(1);
                // panic!();
            }
        }
    }
}

// pub fn get_config() -> PriceScraperConfig {
//     let text = std::fs::read_to_string(
//         r"C:\Users\kestivvi\Desktop\r_prices_workspace\web_scraper\config.json",
//     )
//     .unwrap();
//     let config: PriceScraperConfig = match serde_json::from_str(&text) {
//         Ok(v) => v,
//         Err(e) => panic!("Error parsing config.json, propably some fields missing or there is a typo in the name of a field: {}", e),
//     };
//     config
// }

// pub fn get_selectors() -> HashMap<String, String> {
//     let text = std::fs::read_to_string(
//         r"C:\Users\kestivvi\Desktop\r_prices_workspace\web_scraper\selectors.json",
//     )
//     .unwrap();
//     let selectors: HashMap<String, String> = serde_json::from_str(&text).unwrap();
//     selectors
// }
