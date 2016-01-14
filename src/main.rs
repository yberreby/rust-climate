extern crate chrono;

pub mod location;
pub mod temperature;
pub mod model;

pub use location::{Location, GpsCoordinates};
use model::ModelParams;
use chrono::*;

fn main() {
    let start = Local::now();
    let input = ModelParams {
        location: location::MIAMI,
        date_time: FixedOffset::west(5 * 3600).ymd(2014, 6, 28).and_hms(12, 0, 9),
    };

    let irradiance: f64 = model::irradiance(input);
    let duration = Local::now() - start;

    print!("\n");
    println!("RÉSULTAT :");

    println!("l'irradiance est de {:.2} watts par mètre carré (calculée en {} µs)",
             irradiance,
             duration.num_microseconds().unwrap());
}
