use chrono::*;
use material::Material;
pub use location::{Location, GpsCoordinates};
use temperature::*;

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
    let gmt_offset = date_time.offset().local_minus_utc();
    let day_of_year = date_time.ordinal();
    let eot = equation_of_time(day_of_year);
    let local_meridian_long = local_standard_meridian_longitude(gmt_offset.num_hours() as f64);
    let tcf = time_correction_factor(location.coords.long, local_meridian_long, eot);
    let solar_time = solar_time(date_time.hour() as f64, tcf);
    println!("solar time (hours): {}", solar_time);
    let hour_angle = hour_angle(solar_time);
    println!("hour angle: {}", hour_angle);
    let declination_angle = declination_angle(day_of_year);
    println!("declination angle {}", declination_angle);
    let elevation_angle = elevation_angle(declination_angle, location.coords.lat, hour_angle);
    println!("elevation angle: {}", elevation_angle);
    let zenith_angle = zenith_angle(elevation_angle);
    println!("zenith angle: {}", zenith_angle);
    let air_mass = air_mass(zenith_angle);
    println!("air mass: {}", air_mass);
    let irradiance = irradiance(air_mass);
    println!("irradiance {}", irradiance);

    let kelvin = kelvin_temperature(material.albedo, material.emissivity, irradiance);
    let degrees = kelvin_to_celcius(kelvin);

    degrees
}
