use chrono::*;
pub use location::GpsCoordinates;
use temperature;

pub struct ModelParams<Tz: TimeZone> {
    pub coords: GpsCoordinates,
    pub date_time: DateTime<Tz>,
}

pub struct ModelOutput {
    pub day_of_year: u32,
    pub eot: f64,
    pub local_meridian_long: f64,
    pub time_correction_factor: f64,
    pub solar_time: f64,
    pub hour_angle: f64,
    pub declination_angle: f64,
    pub elevation_angle: f64,
    pub zenith_angle: f64,
    pub air_mass: f64,
    pub irradiance: f64,
}


/// Irradiance, en watts par mètre carré.
pub fn run<Tz: TimeZone>(params: ModelParams<Tz>) -> ModelOutput {
    let ModelParams {
        coords,
        date_time
    } = params;

    let gmt_offset = date_time.offset().local_minus_utc();
    let day_of_year = date_time.ordinal();
    let eot = temperature::equation_of_time(day_of_year);
    let local_meridian_long =
        temperature::local_standard_meridian_longitude(gmt_offset.num_hours() as f64);
    let time_correction_factor =
        temperature::time_correction_factor(coords.long,
                                            local_meridian_long,
                                            eot);
    let solar_time = temperature::solar_time(date_time.hour() as f64,
                                             time_correction_factor);
    let hour_angle = temperature::hour_angle(solar_time);
    let declination_angle = temperature::declination_angle(day_of_year);
    let elevation_angle = temperature::elevation_angle(declination_angle,
                                                       coords.lat,
                                                       hour_angle);
    let zenith_angle = temperature::zenith_angle(elevation_angle);
    let air_mass = temperature::air_mass(zenith_angle);
    let irradiance = temperature::irradiance(air_mass);


    ModelOutput {
        day_of_year: day_of_year,
        eot: eot,
        local_meridian_long: local_meridian_long,
        time_correction_factor: time_correction_factor,
        solar_time: solar_time,
        hour_angle: hour_angle,
        declination_angle: declination_angle,
        elevation_angle: elevation_angle,
        zenith_angle: zenith_angle,
        air_mass: air_mass,
        irradiance: irradiance,
    }
}
