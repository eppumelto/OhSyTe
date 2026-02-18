use chrono::{Datelike, Local};

fn main() {
    let birthdate_string = match env::var("BIRTHDATE") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Virhe: BIRTHDATE-ympäristömuuttujaa ei ole asetettu.");
            return;
        }
    };

    let birth_date = match chrono::NaiveDate::parse_from_str(&birthdate_string, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Virhe: BIRTHDATE-ympäristömuuttuja ei ole oikeassa muodossa (YYYY-MM-DD).");
            return;
        }
    };

    let today = Local::now().date_naive();

    if birth_date.month() == today.month() && birth_date.day() == today.day() {
        println!("Happy birthday!");
    } else {
        println!("Today is not your birthday.");
    }

    let duration = today.signed_duration_since(birth_date);
    let difference = duration.num_days();

    if difference > 0 {
        println!("You are {} days old!", difference);

        if difference % 1000 == 0 {
            println!("That's a nice, round number!");
        } else {
            println!();
        }

    } else if difference < 0 {
        println!("Are you from the future?");
    } else {
        println!("Looks like you're new here.");
    }
}