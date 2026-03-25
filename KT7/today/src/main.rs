use std::fs;
use std::path::PathBuf;

use chrono::{Datelike, Local, NaiveDate};
use today::events::MonthDay;
use today::{create_providers, Config};

fn config_file_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Config directory not found on this OS");
    config_dir.join("today").join("today.toml")
}

fn main() {
    let toml_path = config_file_path();
    let config_str = fs::read_to_string(&toml_path)
        .unwrap_or_else(|_| panic!("Config file not found: {}", toml_path.display()));
    let config: Config = toml::from_str(&config_str).expect("Failed to parse today.toml");

    let config_base_dir = toml_path
        .parent()
        .expect("Invalid config file path; missing parent directory");
    let providers = create_providers(&config, config_base_dir);

    let mut events = Vec::new();
    for provider in providers.iter() {
        provider.get_events(&mut events);
    }

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    println!("Tapahtumat paivalle {}/{}:", today.day(), today.month());

    let mut match_count = 0;
    for event in events {
        if event.month_day() == today_month_day {
            println!("{}", event);
            match_count += 1;
        }
    }

    if match_count == 0 {
        println!("Ei tapahtumia talle paivalle.");
    }
}
