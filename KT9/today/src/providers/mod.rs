use crate::events::{Category, Event};
use chrono::{Local, NaiveDate};

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let other_date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();

        // Täyttää kaikki ehdot
        events.push(Event::new_singular(
            today,
            String::from("Naruappro"),
            Category::from_primary("Opiskelijataphtuma"),
        ));

        // Täyttää päivämäärän ja kategorian, mutta ei kuvausta
        events.push(Event::new_singular(
            today,
            String::from("Sisämaaristeily"),
            Category::from_primary("Opiskelijataphtuma"),
        ));

        // Väärä kategoria
        events.push(Event::new_singular(
            today,
            String::from("Karaokeappro"),
            Category::from_primary("Kierros"),
        ));

        // Väärä päivämäärä
        events.push(Event::new_singular(
            other_date,
            String::from("Joulupukin appro"),
            Category::from_primary("Opiskelijataphtuma"), 
        ));

        // Ei tulostu filtterin läpi
        events.push(Event::new_singular(
            other_date,
            String::from("Joulu Sipoossa"),
            Category::from_primary("juhla"),
        ));
    }
} 