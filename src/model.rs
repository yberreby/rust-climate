use chrono::*;
use material::Material;
pub use location::{Location, GpsCoordinates};

pub struct ModelParams<Tz: TimeZone> {
    pub material: Material,
    pub location: Location,
    pub date_time: DateTime<Tz>,
}

/// Temperature in degrees.
pub fn temperature<Tz: TimeZone>(params: ModelParams<Tz>) -> f64 {
    let ModelParams {
        material,
        location,
        date_time
    } = params;
    let gmt_offset_s = date_time.offset().local_minus_utc();
    // let day_of_year =

    unimplemented!()
}
