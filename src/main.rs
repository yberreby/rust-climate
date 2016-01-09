extern crate chrono;

#[cfg(test)]
extern crate quickcheck;

pub mod location;
pub mod material;
pub mod temperature;
pub mod model;

use std::fmt;
use std::io::{self, Write};
pub use location::{Location, GpsCoordinates};
use material::Material;
use model::ModelParams;
use chrono::*;

fn flush() {
    io::stdout().flush().unwrap();
}


// fn get_input() -> UserInput {
//     unimplemented!()
// }



fn main() {
    let input = ModelParams {
        material: material::CONCRETE,
        location: location::MIAMI,
        date_time: FixedOffset::west(5 * 3600).ymd(2014, 6, 28).and_hms(12, 0, 9),
    };

    let irradiance: f64 = model::irradiance(input);

    print!("\n");
    println!("l'irradiance est de {:.2} watts par mètre carré",
             irradiance);
}
