#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

struct Date {
    year: i16,
    month: Month,
    day: u8,
}

impl Date {
    fn new(year: i16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}

#[derive(Debug)]
struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
}

struct Event {
    date: Date,
    description: String,
    category: Category,
}

impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Event {
            date,
            description,
            category,
        }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay {
            month: self.date.month,
            day: self.date.day,
        }
    }
}


fn main() {
    let events =[
        Event::new(
                Date::new(2001, Month::January, 15),
                String::from("Wikipedia-verkkosanakirja aloitti toimintansa"),
                Category::new("Historia", "Internet"),
            ),
        Event::new(
                Date::new(1919, Month::January, 16),
                String::from("Kieltolaki astui voimaan Yhdysvalloissa"),
                Category::new("Historia", "Yhdysvallat"),
            ),
        Event::new(
                Date::new(1912, Month::January, 17),
                String::from("Robert Falcon Scottin retkikunta saavutti etelänavan"),
                Category::new("Historia", "Tutkimus"),
            ),
        Event::new(
                Date::new(1778, Month::January, 18),
                String::from("Kapteeni James Cook löysi Havaijisaaret"),
                Category::new("Historia", "Tutkimus"),
            ),
        Event::new(
                Date::new(1983, Month::January, 19),
                String::from("Apple julkisti Apple Lisa -tietokoneen"),
                Category::new("Teknologia", "Tietokoneet"),
            ),
        Event::new(
                Date::new(1500, Month::January, 19),
                String::from("Pedro Álvares Cabral löysi Brasilian"),
                Category::new("Historia", "Tutkimus"),
            ),
        Event::new(
                Date::new(1892, Month::January, 20),
                String::from("Ensimmäinen virallinen koripallo-ottelu pelattiin Springfieldissa"),
                Category::new("Urheilu", "Koripallo"),
            ),
        Event::new(
                Date::new(1954, Month::January, 21),
                String::from("Maailman ensimmäinen ydinsukellusvene USS Nautilus laskettiin vesille"),
                Category::new("Teknologia", "Sukellusveneet"),
            ),
        Event::new(
                Date::new(1830, Month::January, 22),
                String::from("Kreikka itsenäistyi Turkista"),
                Category::new("Historia", "Itsenäisyys"),
            ),
        Event::new(
                Date::new(1404, Month::January, 22),
                String::from("Böömin kuningas Wenceslaus IV määräsi Prahan yliopiston perustettavaksi uudelleen"),
                Category::new("Koulutus", "Yliopistot"),
            ),
        Event::new(
                Date::new(1968, Month::January, 23),
                String::from("Tšekkoslovakian kommunistinen puolue aloitti Prahan kevään tukahduttamisen"),
                Category::new("Politiikka", "Kylmä sota"),
            ),
        Event::new(
                Date::new(1973, Month::January, 24),
                String::from("Yhdysvaltain presidentti Richard Nixon nimitti Henry Kissingerin kansalliseksi turvallisuusneuvonantajaksi"),
                Category::new("Politiikka", "Yhdysvallat"),
            ),
        Event::new(
                Date::new(1990, Month::January, 25),
                String::from("Neljä entistä itäsaksalaista osavaltiota liittyi Länsi-Saksaan"),
                Category::new("Historia", "Saksa"),
            ),
        Event::new(
                Date::new(1918, Month::January, 26),
                String::from("Helsingin työväentalon tornissa sytytettiin illalla punainen lyhty vallankumouksen alkamisen merkiksi. Suomen sisällissota syttyi seuraavana päivänä."),
                Category::new("Historia", "Sisällissota"),
            ),
        Event::new(
                Date::new(1924, Month::January, 27),
                String::from("Ensimmäiset talviolympialaiset pidettiin Chamonix'ssa, Ranskassa"),
                Category::new("Urheilu", "Olympialaiset"),
            ),
        Event::new(
                Date::new(1951, Month::January, 28),
                String::from("Yhdysvaltain kongressi hyväksyi 22. perustuslain lisäyksen, joka rajoittaa presidentin virkakaudet kahteen"),
                Category::new("Politiikka", "Yhdysvallat"),
            ),
        Event::new(
                Date::new(1965, Month::January, 29),
                String::from("Tampereella avattiin Suomen ensimmäinen jäähalli Hakametsän halli."),
                Category::new("Ympäristö", "Jäähallit"),
            ),  
    ];

        print!("testausta: ");
        println!();
        let month_day= MonthDay {
            month: Month::January,
            day: 19,
        };

        let mut any_luck = false; 
        for event in events.iter() {
            if event.month_day() == month_day {
                println!(
                    "{}: {} ({:#?})",
                    event.date.year, event.description, event.category
                );
                any_luck = true;
            }
        }

        if !any_luck {
            println!("No events for {:#?}", month_day);
        }
        println!();

        for current_day in month_day.day..=29 {
            println!("Tapahtumat Tammikuun {} päivä:", current_day );
            let mut found = false;
            for event in events.iter(){
                if event.date.day == current_day {
                    println!("{}: {} ({:?})",event.date.year, event.description, event.category);
                    found = true;
                }
            }
            if !found {
                println!("Ei tapahtumia.");
            }
            println!();
        }

        
        

}
