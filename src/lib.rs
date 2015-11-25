extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io;
use hyper::{Url, Client};

pub mod location;
use location::GpsCoordinates;


const TEST_NREL_API_KEY: &'static str = env!("TEST_NREL_API_KEY");

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


//
// Retrieval process:
// INPUT: GPS coordinates, month you're interested in
// OUTPUT: insolation at the given month and location
// * make an API call for the given GPS coords
// * parse the JSON into a Rust struct
// * pick the right month from that struct and return that


pub fn average_ghi(location: &GpsCoordinates, api_key: &str) -> Ghi {
    let client = Client::new();

    let mut url = Url::parse("https://developer.nrel.gov/api/solar/solar_resource/v1.json")
                      .unwrap();

    url.set_query_from_pairs(&[("api_key", api_key),
                               ("lat", &location.lat.to_string()),
                               ("lon", &location.long.to_string())]);

    println!("{:?}", url);


    let res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);


    let json: serde_json::Value = serde_json::de::from_reader(res).unwrap();
    println!("{:#?}", json);

    average_ghi_from_json(&json)

}

/// Parse the JSON returned by NREL's API to extract the average Global Horizontal Irradiance (GHI)
fn average_ghi_from_json(val: &serde_json::Value) -> Ghi {
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



#[test]
fn ghi_test() {
    average_ghi(&location::BOSTON.coords, TEST_NREL_API_KEY);
    average_ghi(&location::DENVER.coords, TEST_NREL_API_KEY);
    average_ghi(&location::LOS_ANGELES.coords, TEST_NREL_API_KEY);
    average_ghi(&location::MIAMI.coords, TEST_NREL_API_KEY);
    average_ghi(&location::SEATTLE.coords, TEST_NREL_API_KEY);
}
