extern crate climate;

use std::io::{self, Write};
use std::collections::HashMap;
use self::climate::location;

fn main() {
    let mut choices = HashMap::new();
    choices.insert("1", location::SEATTLE);
    choices.insert("2", location::BOSTON);
    choices.insert("3", location::LOS_ANGELES);
    choices.insert("4", location::MIAMI);
    choices.insert("5", location::DENVER);

    println!("Merci de choisir un lieu au sein de la liste suivante :\n");

    print!(r##"Villes américaines :
 1. Seattle (Nord-Ouest)
 2. Boston (Nord-Est)
 3. Los Angeles (Sud-Ouest)
 4. Miami (Sud-Est)
 5. Denver (Centre)
    
Entrez le nombre correspondant à la ville désirée : "##);
    io::stdout().flush().unwrap();


    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    println!("");

    let location = match choices.get(choice.trim()) {
        Some(l) => l,
        None => panic!("Mauvais choix"),
    };

    println!("Vous avez choisi : {}", location);

    // match month {
    //    Month::January => ghi.january,
    //    Month::February => ghi.february,
    //    Month::March => ghi.march,
    //    Month::April => ghi.april,
    //    Month::May => ghi.may,
    //    Month::June => ghi.june,
    //    Month::July => ghi.july,
    //    Month::August => ghi.august,
    //    Month::September => ghi.september,
    //    Month::October => ghi.october,
    //    Month::November => ghi.november,
    //    Month::December => ghi.december,
    // }
}
