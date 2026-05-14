use clap::{Parser, Subcommand};
use crate::events::MonthDay; 

#[derive(Parser, Debug)]
#[command(name = "today", author, version, about = "Nayttaa paivan tapahtumat", long_about = None)]
pub struct Cli {
    //Date filter, e.g. "04-02" or "0402" for April 2nd
    #[arg(short = 'd', long = "date")]
    pub date: Option<String>,

    //category filter, e.g. "primary" or "primary/secondary"
    #[arg(short = 'e', long = "exclude")]
    pub exclude: Option<String>,

    /// Ohita ikälaskin ja syntymäpäiväviesti
    #[arg(short = 'n', long = "no-birthday")]
    pub no_birthday: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all event providers
    Providers,

    /// Add an event to a provider
    Add {
        /// Name of the event provider
        #[arg(short = 'p', long = "provider")]
        provider: String,

        /// Date of event (YYYY-MM-DD format)
        #[arg(short = 'd', long = "date")]
        date: String,

        /// Description of event
        #[arg(short = 'e', long = "description")]
        description: String,

        /// Category of event (primary or primary/secondary)
        #[arg(short = 'c', long = "category")]
        category: String,
    },
}

// Parses a date argument in the format "MM-DD" or "MMDD" into a MonthDay struct
pub fn parse_date_arg(date_str: &str) -> Result<MonthDay, String> {
    // removal of - character
    let cleaned = date_str.replace("-", "");
    
    //Lenght check
    if cleaned.len() != 4 {
        return Err(format!("Päivämäärän pitää olla muodossa MM-DD tai MMDD, saatiin: {}", date_str));
    }

    // Parsing month and day, with error handling for non-numeric input
    let month: u32 = cleaned[0..2].parse().map_err(|_| "Virheellinen kuukausi (ei numeroita)")?;
    let day: u32 = cleaned[2..4].parse().map_err(|_| "Virheellinen päivä (ei numeroita)")?;

    Ok(MonthDay::new(month, day))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for parse_date_arg function
    
    #[test]
    fn test_parse_date_arg_with_hyphen() {
        let result = parse_date_arg("04-02").unwrap();
        assert_eq!(result, MonthDay::new(4, 2)); 
    }

    #[test]
    fn test_parse_date_arg_without_hyphen() {
        let result = parse_date_arg("1224").unwrap();
        assert_eq!(result, MonthDay::new(12, 24));
    }

    #[test]
    fn test_parse_date_arg_invalid_length() {
        let result = parse_date_arg("4-2"); // too short
        assert!(result.is_err());
        
        let result_long = parse_date_arg("2024-04-02"); // too long
        assert!(result_long.is_err());
    }

    #[test]
    fn test_parse_date_arg_invalid_characters() {
        let result = parse_date_arg("ab-cd");
        assert!(result.is_err());
    }
}