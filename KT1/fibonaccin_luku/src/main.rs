fn main() {
    //määritellään 101 tulostettavien lukujen määräksi, sillä 0 on ensimmäinen luku
    let n = 101;

    //loopataan n määrä, ja tulostetaan luku kerrallaan
    for i in 0..n {
        println!("{i} = {}", fibonacci(i));
    }
}

//funktio fibonaccin luvun laskemiseen
fn fibonacci(n: u128) -> i128 {
    //jos 0, palautetaan 0
    if n == 0 {
        return 0;
    }

    //muuttujat kahdelle edelliselle luvulle
    let mut a = 0;
    let mut b = 1;

    //lasketaan fibonaccin luku iteroimalla
    for _ in 1..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    //funktion palautusarvo on b, joka sisältää fibonaccin luvun kohdassa n
    b

}
