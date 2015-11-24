const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670367 / 100_000_000.0;

fn kelvin_to_celcius(k: f64) -> f64 {
    k - 273.15
}

fn kelvin_temperature(albedo: f64,
                      emissivity: f64,
                      solar_radiation_watt_per_square_meter: f64)
                      -> f64 {
    assert!(0.0 <= albedo && albedo <= 1.0);
    assert!(0.0 <= emissivity && emissivity <= 1.0);

    (((1.0 - albedo) * solar_radiation_watt_per_square_meter) /
     (4.0 * emissivity * STEFAN_BOLTZMANN_CONSTANT))
        .powf(1.0 / 4.0)
}
