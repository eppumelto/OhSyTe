use std::fmt;
use chrono::{NaiveDate, Datelike};

// Kuukausi ja päivä ilman vuotta
#[derive(Debug, PartialEq)]
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }

    // Odottaa muodossa "MMDD", esim. "0720" = heinäkuu 20.
    pub fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month: u32 = s[..2].parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

// Kategoria voi olla pelkkä pääluokka tai pääluokka + alaluokka (esim. "technology/internet")
#[derive(Debug)]
pub struct Category {
    pub primary: String,
    pub secondary: Option<String>,
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }

    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }

    // Parsii merkkijonon kuten "space/moon landing" kahteen osaan
    pub fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 2 {
            Category { primary: parts[0].to_string(), secondary: None }
        } else {
            Category {
                primary: parts[0].to_string(),
                secondary: Some(parts[1].to_string()),
            }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

// Toistaiseksi vain Singular – tapahtuma joka on tapahtunut kerran tiettynä päivänä
#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub description: String,
    pub category: Category,
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }

    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
            self.description,
            self.category
        )
    }
}