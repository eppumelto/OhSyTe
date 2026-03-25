pub mod data_provider;
pub mod my_events_provider;

use crate::events::Event;

// Kaikki tapahtumatuottajat toteuttavat tämän piirteen
pub trait EventProvider {
    fn name(&self) -> String;
    // Lisää tapahtumat annettuun vektoriin
    fn get_events(&self, events: &mut Vec<Event>);
}