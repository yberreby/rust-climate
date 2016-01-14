extern crate chrono;

#[cfg(test)]
extern crate quickcheck;

pub mod location;
pub mod temperature;
pub mod model;

pub use location::{Location, GpsCoordinates};
use model::ModelParams;
use chrono::*;

fn main() {
    let input = ModelParams {
        location: location::MIAMI,
        date_time: FixedOffset::west(5 * 3600).ymd(2014, 6, 28).and_hms(12, 0, 9),
    };

    let irradiance: f64 = model::irradiance(input);

    print!("\n");
    println!("l'irradiance est de {:.2} watts par mètre carré",
             irradiance);
}
