use crate::events::{Category, Event};
use chrono::NaiveDate;
use std::error::Error;

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
    fn add_event(&self, _date: NaiveDate, _description: String, _category: Category) -> Result<(), Box<dyn Error>> {
        Err("This provider does not support adding events".into())
    }
}