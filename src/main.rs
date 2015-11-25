#![feature(const_fn)]
extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io;
use hyper::{Url, Client};

const API_KEY: &'static str = env!("NREL_API_KEY");

fn main() {

}

//
// Retrieval process:
// INPUT: GPS coordinates, month you're interested in
// OUTPUT: insolation at the given month and location
// * make an API call for the given GPS coords
// * parse the JSON into a Rust struct
// * pick the right month from that struct and return that


fn insolation(lat: f64, long: f64, month: Month) -> f64 {
    let client = Client::new();

    let mut url = Url::parse("https://developer.nrel.gov/api/solar/solar_resource/v1.json")
                      .unwrap();

    // ?api_key=DEMO_KEY&lat=40&lon=-105
    url.set_query_from_pairs(&[("api_key", API_KEY),
                               ("lat", &lat.to_string()),
                               ("lon", &long.to_string())]);

    println!("{:?}", url);


    let res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);


    let json: serde_json::Value = serde_json::de::from_reader(res).unwrap();
    println!("{:#?}", json);

    let ghi = parse_api_json(&json);

    match month {
        Month::January => ghi.january,
        Month::February => ghi.february,
        Month::March => ghi.march,
        Month::April => ghi.april,
        Month::May => ghi.may,
        Month::June => ghi.june,
        Month::July => ghi.july,
        Month::August => ghi.august,
        Month::September => ghi.september,
        Month::October => ghi.october,
        Month::November => ghi.november,
        Month::December => ghi.december,
    }
}


// fn get_insolation_data





/// Average Global Horizontal Irradiance
#[derive(Debug, Clone, Copy)]
pub struct Ghi {
    annual: f64,
    january: f64,
    february: f64,
    march: f64,
    april: f64,
    may: f64,
    june: f64,
    july: f64,
    august: f64,
    september: f64,
    october: f64,
    november: f64,
    december: f64,
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

fn parse_api_json(val: &serde_json::Value) -> Ghi {
    let avg_ghi = val.find_path(&["outputs", "avg_ghi"])
                     .and_then(|x| x.as_object())
                     .unwrap();

    let months = avg_ghi.get("monthly").and_then(|x| x.as_object()).unwrap();

    let get_month = |month| months.get(month).and_then(|x| x.as_f64()).unwrap();

    Ghi {
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


pub struct Location {
    lat: f64,
    long: f64,
}

pub fn location(lat: f64, long: f64) -> Location {
    assert!(-90.0 <= lat && lat <= 90.0);
    assert!(-180.0 <= long && long <= 180.0);

    Location {
        lat: lat,
        long: long,
    }
}

// coordinates from Google

// north-west
pub const SEATTLE: Location = Location {
    lat: 47.6097,
    long: 122.3331,
};
// north-east
pub const BOSTON: Location = Location {
    lat: 42.3601,
    long: 71.0589,
};

// south-west
pub const LOS_ANGELES: Location = Location {
    lat: 34.0500,
    long: 118.2500,
};

// south-east
pub const MIAMI: Location = Location {
    lat: 25.7753,
    long: 80.2089,
};

// center
pub const DENVER: Location = Location {
    lat: 39.7392,
    long: 104.9903,
};

#[test]
fn insolation_test() {
    insolation(37.0, -122.0, Month::January);
}
