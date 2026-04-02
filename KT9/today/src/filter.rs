use crate::events::{Event, Category, MonthDay};

#[derive(Debug, Default, Clone)]
pub struct EventFilter {
    pub date: Option<MonthDay>,
    pub category: Option<Category>,
    pub description_substring: Option<String>,
}

pub struct EventFilterBuilder {
    filter: EventFilter,
}

impl EventFilter {
    pub fn builder() -> EventFilterBuilder {
        EventFilterBuilder {
            filter: EventFilter::default(),
        }
    }
    
    // Testataan tapahtuman sopivuutta suodattimeen
    pub fn matches(&self, event: &Event) -> bool {
        // Jos päivämäärä on määritetty ja se ei täsmää, palauta false
        if let Some(date) = &self.date {
            if &event.month_day() != date {
                return false;
            }
        }
        //jos kategoria ei täsmää
        if let Some(category) = &self.category {
            if &event.category != category {
                return false;
            }
        }
        
        // Jos kuvaus ei sisällä määrättyä stringiä
        if let Some(substring) = &self.description_substring {
            if !event.description.contains(substring) {
                return false;
            }
        }
        
        // Jos ehtoja 0 tai kaikki läpi
        true
    }
}

impl EventFilterBuilder {
    //Builderi joka ketjuttaa suodattimen määrittelyä
    pub fn empty() -> Self {
        Self { filter: EventFilter::default() }
    }

    pub fn date(mut self, date: MonthDay) -> Self {
        self.filter.date = Some(date);
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.filter.category = Some(category);
        self
    }

    pub fn description_contains(mut self, substring: &str) -> Self {
        self.filter.description_substring = Some(substring.to_string());
        self
    }

    pub fn build(self) -> EventFilter {
        self.filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn sample_event() -> Event {
        Event::new_singular(
            NaiveDate::from_ymd_opt(2003, 3, 19).unwrap(),
            "Irakin sota".to_string(),
            Category::new("history", "NULL"),
        )
    }

    #[test]
    fn test_filter_empty_matches_all() {
        let event = sample_event();
        let filter = EventFilter::builder().build();
        assert!(filter.matches(&event));
    }

    #[test]
    fn test_filter_date_matches() {
        let event = sample_event();
        
        // Etsitään 19. maaliskuuta
        let filter_match = EventFilter::builder().date(MonthDay::new(3, 19)).build();
        // Etsitään 1. tammikuuta 
        let filter_mismatch = EventFilter::builder().date(MonthDay::new(1, 1)).build();
        
        assert!(filter_match.matches(&event));
        assert!(!filter_mismatch.matches(&event));
    }

    #[test]
    fn test_filter_combined_criteria() {
        let event = sample_event();
        
        // Käytetään sample_eventin tietoja
        let filter = EventFilter::builder()
            .date(MonthDay::new(3, 19))
            .description_contains("Irakin")
            .build();
            
        assert!(filter.matches(&event));
    }
}