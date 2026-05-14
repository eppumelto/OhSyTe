use std::path::PathBuf;
use std::fs;
use dirs;
use chrono::NaiveDate;

use clap::Parser;
use today::cli::{Cli, Commands, parse_date_arg};
use today::events::Category;
use today::{add_event_to_provider, run, Config};

fn main() {
    let args = Cli::parse();

    if !args.no_birthday {
        today::birthday::calculate_age_and_print();
    }

    // Paivamaaran kasittely
    let target_date = match args.date {
        Some(date_str) => match parse_date_arg(&date_str) {
            Ok(parsed_date) => Some(parsed_date),
            Err(e) => {
                eprintln!("Virhe argumenteissa: {}", e);
                std::process::exit(1);
            }
        },
        None => None,
    };

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME).expect("Ei voitu luoda tai hakea asetuspolkua");
    let toml_path = config_path.join(format!("{}.toml", APP_NAME));
    
    let config_str = fs::read_to_string(&toml_path).expect("Asetustiedostoa ei löytynyt");
    let config: Config = toml::from_str(&config_str).expect("Asetustiedosto on virheellinen");

    match args.command {
        Some(Commands::Providers) => {
            println!("Tuetut tapahtumatuottajat:");
            for p in &config.providers {
                println!("- {} ({})", p.name, p.kind);
            }
        }
        Some(Commands::Add { provider, date, description, category }) => {
            let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => {
                    eprintln!("Virhe: paivamaaran formaatti on YYYY-MM-DD");
                    std::process::exit(1);
                }
            };

            let parsed_category = Category::from_str(&category);

            match add_event_to_provider(
                &config,
                &config_path,
                &provider,
                parsed_date,
                description,
                parsed_category,
            ) {
                Ok(_) => {
                    println!("Tapahtuma lisatty tuottajalle '{}'", provider);
                }
                Err(e) => {
                    eprintln!("Virhe tapahtuman lisayksessa: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            if let Err(e) = run(&config, &config_path, target_date, args.exclude) {
                eprintln!("Virhe suorituksessa: {}", e);
            }
        }
    }
}

// Gets the configuration directory path for the application
// named in the `app_name` argument.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or None if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
}