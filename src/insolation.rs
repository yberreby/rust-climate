extern crate hyper;

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

/// Get the insolation at the specified location and month
pub fn insolation(lat: f64, long: f64, month: Month) -> f64 {
    unimplemented!()
}
