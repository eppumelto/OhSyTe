pub mod events;
pub mod providers;

use serde::Deserialize;
use std::path::Path;

use providers::my_events_provider::MyEventsProvider;
use providers::EventProvider;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

pub fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();

    for cfg in config.providers.iter() {
        let _path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "my_events" => providers.push(Box::new(MyEventsProvider)),
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }
    providers
}