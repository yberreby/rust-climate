extern crate chrono;

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
        date_time: UTC.ymd(2014, 6, 28).and_hms(12, 0, 9),
    };

    let temperature: f64 = model::temperature(input);

    println!("La température de l'objet est : {}°C", temperature);
}
