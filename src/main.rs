extern crate hyper;
extern crate serde;
extern crate serde_json;

use hyper::{Url, Client};

pub mod location;
use location::GpsCoordinates;


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

//
// Retrieval process:
// INPUT: GPS coordinates, month you're interested in
// OUTPUT: insolation at the given month and location
// * make an API call for the given GPS coords
// * parse the JSON into a Rust struct
// * pick the right month from that struct and return that


pub fn average_ghi(location: &GpsCoordinates) -> CompleteGhi {
    let client = Client::new();

    let mut url = Url::parse("https://developer.nrel.gov/api/solar/solar_resource/v1.json")
                      .unwrap();

    url.set_query_from_pairs(&[("api_key", NREL_API_KEY),
                               ("lat", &location.lat.to_string()),
                               ("lon", &location.long.to_string())]);

    println!("{:?} {}", url, url.serialize());

    let res = client.get(url).send().unwrap();
    println!("{:?}", res);
    assert_eq!(res.status, hyper::Ok);


    let json: serde_json::Value = serde_json::de::from_reader(res).unwrap();

    average_ghi_from_json(&json)

}

/// Parse the JSON returned by NREL's API to extract the average Global Horizontal Irradiance (GHI)
fn average_ghi_from_json(val: &serde_json::Value) -> CompleteGhi {
    let avg_ghi = val.find_path(&["outputs", "avg_ghi"])
                     .and_then(|x| x.as_object())
                     .unwrap();

    let months = avg_ghi.get("monthly").and_then(|x| x.as_object()).unwrap();

    let get_month = |month| months.get(month).and_then(|x| x.as_f64()).unwrap();

    CompleteGhi {
        annual: avg_ghi.get("annual").and_then(|x| x.as_f64()).unwrap(),
        january: get_month("jan"),
        february: get_month("feb"),
        march: get_month("mar"),
        april: get_month("apr"),
        may: get_month("may"),
        june: get_month("jun"),
        july: get_month("jul"),
        august: get_month("aug"),
        september: get_month("sep"),
        october: get_month("oct"),
        november: get_month("nov"),
        december: get_month("dec"),
    }
}



#[test]
fn ghi_test() {
    average_ghi(&location::BOSTON.coords);
    average_ghi(&location::DENVER.coords);
    average_ghi(&location::LOS_ANGELES.coords);
    average_ghi(&location::MIAMI.coords);
    average_ghi(&location::SEATTLE.coords);
}
use std::io::{self, Write};
use std::fmt;


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
                   Month::March => "march",
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

    let avg_ghi = average_ghi(&location.coords);
    println!("ok.\n");

    print!("Entrez un mois (nombre de 1 à 12) : ");
    flush();

    io::stdin().read_line(&mut choice).unwrap();

    let idx = choice.trim().parse::<usize>().unwrap() - 1;
    let month = months.get(idx).unwrap();

    println!("GHI à {} au mois de {} : {}",
             location,
             month,
             average_ghi_for_month(&avg_ghi, &month));

    println!("Données : {:?}", avg_ghi);
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
