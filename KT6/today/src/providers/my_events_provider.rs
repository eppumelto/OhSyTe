use chrono::NaiveDate;
use crate::events::{Event, Category};
use super::EventProvider;

pub struct MyEventsProvider;

impl EventProvider for MyEventsProvider {
    fn name(&self) -> String {
        "My Events Provider".to_string()
    }

    // Lista tapahtumista
    fn get_events(&self, events: &mut Vec<Event>) {
        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(1969, 7, 20).unwrap(),
            "Apollo 11 lands on the Moon".to_string(),
            Category::new("space", "moon landing"),
        ));

        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(1989, 11, 9).unwrap(),
            "Berlin Wall falls".to_string(),
            Category::from_primary("history"), 
        ));

        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(1991, 8, 6).unwrap(),
            "World Wide Web becomes publicly available".to_string(),
            Category::new("technology", "internet"),
        ));

        events.push(Event::new_singular(
            NaiveDate::from_ymd_opt(2004, 2, 4).unwrap(),
            "Facebook is founded".to_string(),
            Category::new("technology", "social media"),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_count() {
        let provider = MyEventsProvider;
        let mut events = Vec::new();
        provider.get_events(&mut events);
        assert_eq!(events.len(), 4);
    }

    #[test]
    fn test_provider_name() {
        let provider = MyEventsProvider;
        assert_eq!(provider.name(), "My Events Provider");
    }

    #[test]
    fn test_first_event() {
        let provider = MyEventsProvider;
        let mut events = Vec::new();
        provider.get_events(&mut events);
        let first = &events[0];
        assert_eq!(first.description, "Apollo 11 lands on the Moon");
        assert_eq!(first.year(), 1969);
        let md = first.month_day();
        assert_eq!(md.month, 7);
        assert_eq!(md.day, 20);
    }

    #[test]
    fn test_category_of_second_event() {
        let provider = MyEventsProvider;
        let mut events = Vec::new();
        provider.get_events(&mut events);
        let second = &events[1];
        assert_eq!(second.category.primary, "history");
        assert!(second.category.secondary.is_none());
    }
}
