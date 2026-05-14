use chrono::{NaiveDate, Local, Datelike};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::events::{Category, Event, MonthDay};
use crate::providers::EventProvider;

pub struct WebProvider {
    name: String,
    url: String,
}
// This provider retrieves events from a web API. It expects the API to return JSON data in a specific format, which is deserialized into Rust structs. The provider constructs the API URL using the current date (month and day) as a query parameter, and then parses the response to create Event instances.
impl WebProvider {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string()
        }
    }
}

#[derive(Deserialize, Debug)]
struct HistoryResponse {
    data: HistoryData,
}

#[derive(Deserialize, Debug)]
struct HistoryData {
    #[serde(rename = "Events")]
    events: Vec<HistoryEvent>,
}

#[derive(Deserialize, Debug)]
struct HistoryEvent {
    year: String,
    text: String,
}

impl EventProvider for WebProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let month_day = MonthDay::new(today.month(), today.day());
        let date_parameter = format!(
            "date={:02}-{:02}",
            month_day.month(),
            month_day.day());
        let client = Client::new();
        let url = format!("{}?{}", &self.url, date_parameter);
        let response = match client.get(&url).send() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error while retrieving data from '{}': {}", self.name, e);
                return;
            }
        };

        let response_body = match response.json::<HistoryResponse>() {
            Ok(body) => body,
            Err(e) => {
                eprintln!("Invalid JSON in web provider '{}': {}", self.name, e);
                return;
            }
        };
        println!("Got {} events from JSON", response_body.data.events.len());

        for json_event in response_body.data.events {
            let year = match json_event.year.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let date = match NaiveDate::from_ymd_opt(year, month_day.month(), month_day.day()) {
                Some(v) => v,
                None => continue,
            };
            let category = Category::from_primary("history");
            let event = Event::new_singular(date, json_event.text, category);
            events.push(event);
        }
    }
}