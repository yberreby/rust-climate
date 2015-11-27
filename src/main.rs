use std::fmt;
use std::io::{self, Write};

pub mod location;
pub use location::GpsCoordinates;

pub mod nrel;

pub mod material;
pub mod temperature;

const NREL_API_KEY: &'static str = env!("NREL_API_KEY");

pub type Ghi = f64;

/// Average Global Horizontal Irradiance
#[derive(Debug, Clone, Copy)]
pub struct CompleteGhi {
    pub annual: Ghi,
    pub january: Ghi,
    pub february: Ghi,
    pub march: Ghi,
    pub april: Ghi,
    pub may: Ghi,
    pub june: Ghi,
    pub july: Ghi,
    pub august: Ghi,
    pub september: Ghi,
    pub october: Ghi,
    pub november: Ghi,
    pub december: Ghi,
}




pub enum Month {
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

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Month::January => "janvier",
                   Month::February => "février",
                   Month::March => "mars",
                   Month::April => "avril",
                   Month::May => "mai",
                   Month::June => "juin",
                   Month::July => "juillet",
                   Month::August => "août",
                   Month::September => "septembre",
                   Month::October => "octobre",
                   Month::November => "novembre",
                   Month::December => "décembre",
               })
    }
}

fn flush() {
    io::stdout().flush().unwrap();
}

fn average_ghi_for_month(complete_ghi: &CompleteGhi, month: &Month) -> f64 {
    match *month {
        Month::January => complete_ghi.january,
        Month::February => complete_ghi.february,
        Month::March => complete_ghi.march,
        Month::April => complete_ghi.april,
        Month::May => complete_ghi.may,
        Month::June => complete_ghi.june,
        Month::July => complete_ghi.july,
        Month::August => complete_ghi.august,
        Month::September => complete_ghi.september,
        Month::October => complete_ghi.october,
        Month::November => complete_ghi.november,
        Month::December => complete_ghi.december,
    }
}

fn main() {
    let cities = vec![location::SEATTLE,
                      location::BOSTON,
                      location::LOS_ANGELES,
                      location::MIAMI,
                      location::DENVER];

    let months = vec![Month::January,
                      Month::February,
                      Month::March,
                      Month::April,
                      Month::May,
                      Month::June,
                      Month::July,
                      Month::August,
                      Month::September,
                      Month::October,
                      Month::November,
                      Month::December];

    let materials = vec![material::SAND,
                         material::GREEN_GRASS,
                         material::STAINLESS_STEEL,
                         material::CONCRETE];

    println!("Merci de choisir un lieu au sein de la liste suivante :\n");

    print!(r##"Villes américaines :
 1. Seattle (Nord-Ouest)
 2. Boston (Nord-Est)
 3. Los Angeles (Sud-Ouest)
 4. Miami (Sud-Est)
 5. Denver (Centre)

Entrez le nombre correspondant à la ville désirée : "##);
    flush();


    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    println!("");

    let idx = choice.trim().parse::<usize>().unwrap() - 1; // UX uses 1-indexed lists
    let location = cities.get(idx).unwrap();

    println!("Vous avez choisi : {}\n", location);

    print!("Recherche de données d'ensoleillement dans la base NREL... ");
    flush();

    let avg_ghi = nrel::average_ghi(&location.coords);
    println!("ok.\n");

    print!("Entrez un mois (nombre de 1 à 12) : ");
    flush();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let idx = choice.trim().parse::<usize>().unwrap() - 1;
    let month = months.get(idx).unwrap();

    let ghi = average_ghi_for_month(&avg_ghi, &month);

    println!("Global Horizontal Irradiance à {} au mois de '{}' : {:.2} kWh/m2/jour",
             location,
             month,
             ghi);

    println!("Merci de choisir un matériau au sein de la liste suivante :\n");

    print!(r##"Matériaux :
 1. Sable
 2. Herbe verte
 3. Acier inoxydable 
 4. Béton

Entrez le nombre correspondant au matériau désiré : "##);
    flush();


    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let idx = choice.trim().parse::<usize>().unwrap() - 1;
    let material = materials.get(idx).unwrap();

    println!("Vous avez chosi le matériau : {}", material);

    let temperature =
        temperature::kelvin_to_celcius(temperature::kelvin_temperature(material.albedo,
                                                                       material.emissivity,
                                                                       ghi * 1000.0 / 24.0)); // 1366

    println!("Température : {:.2}°C", temperature);

    // println!("Données : {:?}", avg_ghi);
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
