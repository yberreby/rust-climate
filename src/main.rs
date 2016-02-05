extern crate chrono;

pub mod location;
pub mod irradiance;
pub mod model;

pub use location::GpsCoordinates;
use model::ModelParams;
use chrono::*;
use std::io::{self, Write, BufRead};

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

    let local_time = DateTime::parse_from_str(&local_time_string,
                                              "%d/%m/%Y %H:%M:%S %z")
                         .unwrap();

    let coords = GpsCoordinates {
        lat: lat,
        long: long,
    };

    let input = ModelParams {
        coords: coords,
        date_time: local_time,
    };

    let start = Local::now();
    let output = model::run(input);
    let duration = Local::now() - start;

    println!("==== VARIABLES INTERMÉDIAIRES ====");
    println!("jour de l'année : {}", output.day_of_year);
    println!("équation du temps : {:.2} minutes", output.eot);
    println!("longitude du méridien local : {}°",
             output.local_meridian_long);
    println!("facteur de correction de l'heure : {:.2} minutes",
             output.time_correction_factor);
    println!("heure solaire : {:.2}h", output.solar_time);
    println!("angle horaire : {:.4}°", output.hour_angle);
    println!("angle de déclinaison de la Terre : {:.4}°",
             output.declination_angle);
    println!("angle d'élévation : {:.4}°", output.elevation_angle);
    println!("angle zénithal : {:.4}°", output.zenith_angle);
    println!("masse d'air : {:.4}", output.air_mass);
    print!("\n");

    println!("==== RÉSULTAT ====");

    println!("l'éclairement énergétique solaire est de {:.2} watts par mètre \
              carré (calculé en {} µs)",
             output.irradiance,
             duration.num_microseconds().unwrap());
}
