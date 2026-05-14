use std::path::{Path, PathBuf};
use std::collections::HashMap;
use sqlite::{Connection, State};
use chrono::NaiveDate;
use crate::events::{Event, Category};
use crate::providers::EventProvider;

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf()
        }
    }

    fn read_categories(
        &self,
        connection: &Connection,
        query: &str,
        id_column: &str,
        primary_column: &str,
        secondary_column: Option<&str>,
    ) -> Option<HashMap<i64, Category>> {
        let mut category_map: HashMap<i64, Category> = HashMap::new();
        let mut statement = match connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(_) => return None,
        };

        while let Ok(State::Row) = statement.next() {
            let category_id = match statement.read::<i64, _>(id_column) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let primary = match statement.read::<String, _>(primary_column) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let category = match secondary_column {
                Some(column) => match statement.read::<Option<String>, _>(column) {
                    Ok(Some(sec)) => Category::new(&primary, &sec),
                    Ok(None) => Category::from_primary(&primary),
                    Err(_) => continue,
                },
                None => Category::from_primary(&primary),
            };
            category_map.insert(category_id, category);
        }

        Some(category_map)
    }

    //read categories and put them in hashmap
    fn get_categories(&self, connection: &Connection) -> HashMap<i64, Category> {
        if let Some(category_map) = self.read_categories(
            connection,
            "SELECT category_id, primary_name, secondary_name FROM category",
            "category_id",
            "primary_name",
            Some("secondary_name"),
        ) {
            return category_map;
        }

        self.read_categories(
            connection,
            "SELECT id, name FROM category",
            "id",
            "name",
            None,
        )
        .unwrap_or_default()
    }

    fn read_events(
        &self,
        connection: &Connection,
        category_map: &HashMap<i64, Category>,
        events: &mut Vec<Event>,
        query: &str,
        date_column: &str,
        description_column: &str,
        category_column: &str,
    ) -> bool {
        let mut statement = match connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(_) => return false,
        };

        while let Ok(State::Row) = statement.next() {
            let date_string = match statement.read::<String, _>(date_column) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let date = match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(v) => v,
                Err(_) => continue,
            };
            let description = match statement.read::<String, _>(description_column) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let category_id = match statement.read::<i64, _>(category_column) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(category) = category_map.get(&category_id) {
                events.push(Event::new_singular(date, description.to_string(), category.clone()));
            }
        }

        true
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    //read events and put them in events vector
    fn get_events(&self, events: &mut Vec<Event>) {
        let connection = match Connection::open(self.path.clone()) {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("SQLite provider '{}': failed to open DB: {}", self.name, e);
                return;
            }
        };

        let category_map = self.get_categories(&connection);
        if self.read_events(
            &connection,
            &category_map,
            events,
            "SELECT event_date, event_description, category_id FROM event",
            "event_date",
            "event_description",
            "category_id",
        ) {
            return;
        }

        let _ = self.read_events(
            &connection,
            &category_map,
            events,
            "SELECT date, description, category_id FROM event",
            "date",
            "description",
            "category_id",
        );
    }
}