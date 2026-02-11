fn main() {
let events = [
        (2001, 15, "Wikipedia-verkkosanakirja aloitti toimintansa"),
        (1919, 16, "Kieltolaki astui voimaan Yhdysvalloissa"),
        (1912, 17, "Robert Falcon Scottin retkikunta saavutti etelänavan"),
        (1778, 18, "Kapteeni James Cook löysi Havaijisaaret"),
        (1983, 19, "Apple julkisti Apple Lisa -tietokoneen"),
        (1500, 19, "Pedro Álvares Cabral löysi Brasilian"),
        (1892, 20, "Ensimmäinen virallinen koripallo-ottelu pelattiin Springfieldissa"),
        (1954, 21, "Maailman ensimmäinen ydinsukellusvene USS Nautilus laskettiin vesille"),
        (1830, 22, "Kreikka itsenäistyi Turkista"),
        (1404, 22, "Böömin kuningas Wenceslaus IV määräsi Prahan yliopiston perustettavaksi uudelleen")
    ];

    print!("testausta: ");
    println!();
    let test = 19;
    for (year, day, event) in events.iter() {
        if *day == test {

            println!("{}: {}", year, event);
        }
    }

    println!();

    for current_day in 15..=22{
        println!("Tapahtumat Tammikuun {} päivä:", current_day );
        for (y,d,e) in events.iter(){
            if *d == current_day {
                println!("{}: {}",y, e);
            }
        }
    }

}
