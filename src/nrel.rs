extern crate hyper;
extern crate serde;
extern crate serde_json;

use self::hyper::{Url, Client};

use GpsCoordinates;
use CompleteGhi;

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

    url.set_query_from_pairs(&[("api_key", ::NREL_API_KEY),
                               ("lat", &location.lat.to_string()),
                               ("lon", &location.long.to_string())]);

    // println!("{:?} {}", url, url.serialize());

    let res = client.get(url).send().unwrap();
    // println!("{:?}", res);
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


#[cfg(test)]
mod tests {
    use location;
    use super::average_ghi;

    #[test]
    fn ghi_test() {
        average_ghi(&location::BOSTON.coords);
        average_ghi(&location::DENVER.coords);
        average_ghi(&location::LOS_ANGELES.coords);
        average_ghi(&location::MIAMI.coords);
        average_ghi(&location::SEATTLE.coords);
    }
}
