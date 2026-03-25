use today::events::Event;
use today::providers::EventProvider;
use today::providers::my_events_provider::MyEventsProvider;

fn main() {
    let provider = MyEventsProvider;
    let mut events: Vec<Event> = Vec::new();
    provider.get_events(&mut events);
    for event in &events {
        println!("{}", event);
    }
}
