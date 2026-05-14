use std::error::Error;
use std::path::Path;

pub mod events;
pub mod providers;
pub mod filter;
pub mod cli;
pub mod birthday;

use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;

use crate::filter::EventFilter;

use events::{Category, Event, MonthDay};
use providers::EventProvider;
use crate::providers::{
    csvfile::CSVFileProvider,
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider
};

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    providers
}

pub fn run(config: &Config, config_path: &Path, date: Option<MonthDay>, exclude: Option<String>) -> Result<(), Box<dyn Error>> {
    
    let mut events: Vec<Event> = Vec::new();

    // Parse excluded categories
    let excluded_categories: Vec<Category> = exclude
        .map(|exclude_str| {
            exclude_str
                .split(',')
                .filter(|cat| !cat.trim().is_empty())
                .map(|cat| Category::from_str(cat.trim()))
                .collect()
        })
        .unwrap_or_default();

    let event_filter = match date {
        Some(d) => EventFilter::builder()
            .date(d)
            .exclude_categories(excluded_categories)
            .build(),
        None => {
            let today: NaiveDate = Local::now().date_naive();
            let today_month_day = MonthDay::new(today.month(), today.day());
            EventFilter::builder()
                .date(today_month_day)
                .exclude_categories(excluded_categories)
                .build()
        }
    };

    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events);  // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    for event in events {
        if event_filter.matches(&event) {
            println!("{}", event);
        }
    }

    Ok(())
}

pub fn add_event_to_provider(
    config: &Config,
    config_path: &Path,
    provider_name: &str,
    date: NaiveDate,
    description: String,
    category: Category,
) -> Result<(), Box<dyn Error>> {
    let providers = create_providers(config, config_path);
    for provider in providers {
        if provider.name() == provider_name {
            return provider.add_event(date, description, category);
        }
    }

    Err(format!("Provider '{}' ei loytynyt", provider_name).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_add_event_to_csv_provider() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("today_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();

        let config = Config {
            providers: vec![ProviderConfig {
                name: "csvtest".to_string(),
                kind: "csv".to_string(),
                resource: "events.csv".to_string(),
            }],
        };

        let result = add_event_to_provider(
            &config,
            &temp_dir,
            "csvtest",
            NaiveDate::from_ymd_opt(2024, 5, 10).unwrap(),
            "Test event".to_string(),
            Category::from_primary("test"),
        );

        assert!(result.is_ok());

        let written = fs::read_to_string(temp_dir.join("events.csv")).unwrap();
        assert!(written.contains("2024-05-10,Test event,test"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}