use chrono::NaiveTime;

const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670367 / 100_000_000.0;

pub fn kelvin_to_celcius(k: f64) -> f64 {
    k - 273.15
}

pub fn kelvin_temperature(albedo: f64,
                          emissivity: f64,
                          solar_radiation_watt_per_square_meter: f64)
                          -> f64 {
    assert!(0.0 <= albedo && albedo <= 1.0);
    assert!(0.0 <= emissivity && emissivity <= 1.0);

    (((1.0 - albedo) * solar_radiation_watt_per_square_meter) /
     (4.0 * emissivity * STEFAN_BOLTZMANN_CONSTANT))
        .powf(1.0 / 4.0)
}


// WIP

// need day_of_year()
// -> .ordinal() (1-indexed)

// l'ensoleillement (insolation, english) est une mesure d'énergie
// l'irradiance (irradiance, english) est une mesure de puissance

/// Irradiance d'un point, en Watts par mètre carré, en fonction de la masse d'air
/// devant être traversée par les rayons du Soleil.
pub fn irradiance(air_mass: f64) -> f64 {
    1.353 * 0.7f64.powf(air_mass.powf(0.678))
}

pub fn air_mass(zenith_angle: f64) -> f64 {
    1.0 / zenith_angle.cos()
}

/// Angle zénithal en fonction de l'angle d'élévation.
pub fn zenith_angle(elevation_angle: f64) -> f64 {
    90.0 - elevation_angle
}

/// Angle d'élévation en fonction de l'angle de déclinaison, de la latitude et de l'angle horaire.
pub fn elevation_angle(declination_angle: f64, latitude: f64, hour_angle: f64) -> f64 {
    (declination_angle.sin() * latitude.sin() +
     declination_angle.cos() * latitude.cos() * hour_angle.cos())
        .asin()
}

/// Angle de déclinaison en fonction du jour de l'année.
pub fn declination_angle(day_of_year: u32) -> f64 {
    assert!(1 <= day_of_year && day_of_year <= 366); // leap years

    (23.45f64.sin() * ((360.0 / 365.0) * (day_of_year - 81) as f64).sin()).asin()
}

/// Angle horaire en fonction de l'heure solaire.
pub fn hour_angle(solar_time: f64) -> f64 {
    15.0 * (solar_time - 12.0)
}

/// Heure solaire en fonction de l'heure locale et du facteur de correction de l'heure.
pub fn solar_time(local_time: f64, time_correction_factor: f64) -> f64 {
    local_time + time_correction_factor / 60.0
}

/// Facteur de correction de l'heure en fonction de la longitude du point étudié,
/// de la longitude du méridien standard local et de l'équation du temps.
pub fn time_correction_factor(longitude: f64,
                              local_standard_meridian_longitude: f64,
                              equation_of_time: f64)
                              -> f64 {
    4.0 * (longitude - local_standard_meridian_longitude) + equation_of_time
}

/// Équation du temps en fonction du jour de l'année.
pub fn equation_of_time(day_of_year: u32) -> f64 {
    let b = (360.0 / 365.0) * (day_of_year - 81) as f64;
    9.87 * (2.0 * b).sin() - 7.53 * b.cos() - 1.5 * b.sin()
}
