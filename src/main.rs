extern crate chrono;

pub mod location;
pub mod temperature;
pub mod model;

pub use location::{Location, GpsCoordinates};
use model::ModelParams;
use chrono::*;
use std::io::{self, Read, Write, BufRead};

fn flush() {
    io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // User input.

    println!("==== PARAMÈTRES D'ENTRÉE ====");
    print!("latitude (en degrés) : ");
    flush();
    let lat: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    print!("longitude (en degrés) : ");
    flush();
    let long: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    print!("date et heure locale (e.g. \"28/11/2016 21:00:09 +09:00\"): ");
    flush();

    let local_time_string = lines.next().unwrap().unwrap();
    println!("");

    // Model.

    let local_time = DateTime::parse_from_str(&local_time_string, "%d/%m/%Y %H:%M:%S %z").unwrap();

    let coords = GpsCoordinates {
        lat: lat,
        long: long,
    };

    let input = ModelParams {
        coords: coords,
        date_time: local_time,
    };

    let start = Local::now();
    let irradiance: f64 = model::irradiance(input);
    let duration = Local::now() - start;

    print!("\n");
    println!("==== RÉSULTAT ====");

    println!("l'irradiance est de {:.2} watts par mètre carré (calculée en {} µs)",
             irradiance,
             duration.num_microseconds().unwrap());
}
