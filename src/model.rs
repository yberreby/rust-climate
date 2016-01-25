use chrono::*;
pub use location::GpsCoordinates;
use temperature;

pub struct ModelParams<Tz: TimeZone> {
    pub coords: GpsCoordinates,
    pub date_time: DateTime<Tz>,
}

/// Irradiance, en watts par mètre carré.
pub fn irradiance<Tz: TimeZone>(params: ModelParams<Tz>) -> f64 {
    let ModelParams {
        coords,
        date_time
    } = params;

    let gmt_offset = date_time.offset().local_minus_utc();
    let day_of_year = date_time.ordinal();
    let eot = temperature::equation_of_time(day_of_year);
    let local_meridian_long =
        temperature::local_standard_meridian_longitude(gmt_offset.num_hours() as f64);
    let tcf = temperature::time_correction_factor(coords.long, local_meridian_long, eot);
    let solar_time = temperature::solar_time(date_time.hour() as f64, tcf);
    let hour_angle = temperature::hour_angle(solar_time);
    let declination_angle = temperature::declination_angle(day_of_year);
    let elevation_angle = temperature::elevation_angle(declination_angle, coords.lat, hour_angle);
    let zenith_angle = temperature::zenith_angle(elevation_angle);
    let air_mass = temperature::air_mass(zenith_angle);
    let irradiance = temperature::irradiance(air_mass);

    // println!("PARAMÈTRES D'ENTRÉE :");
    // println!("emplacement : {}", location);
    // println!("heure et date : {}",
    //          date_time.naive_local().format("%H:%M:%S %d/%m/%Y").to_string());
    // println!("décalage horaire : {} heure(s) par rapport à l'heure GMT",
    //          gmt_offset.num_hours());
    // print!("\n");

    println!("==== VARIABLES INTERMÉDIAIRES ====");
    println!("jour de l'année : {}", day_of_year);
    println!("équation du temps : {:.2} minutes", eot);
    println!("longitude du méridien local : {}°", local_meridian_long);
    println!("facteur de correction de l'heure : {:.2} minutes", tcf);
    println!("heure solaire : {:.2}h", solar_time);
    println!("angle horaire : {:.4}°", hour_angle);
    println!("angle de déclinaison de la Terre : {:.4}°",
             declination_angle);
    println!("angle d'élévation : {:.4}°", elevation_angle);
    println!("angle zénithal : {:.4}°", zenith_angle);
    println!("coefficient de masse atmosphérique : {:.4}", air_mass);

    irradiance
}
