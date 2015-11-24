use std::io;

//
// fn main() {
//    let temp_c = kelvin_to_celcius(kelvin_temperature(0.6, 0.966, 1000.0));
//    println!("Temp: {}°C", temp_c);
//    println!("Constant: {}", STEFAN_BOLTZMANN_CONSTANT);
// }


mod model;
// mod materials;
mod insolation;

// use model::ModelParams;

type Insolation = f64; // watts (power unit) per square meter
type Emissivity = f64; // emissivity coefficient - a ratio (0 to 1, float)
type Albedo = f64; //





//
// We'll have two big parts: input and model
// model must be decoupled from input
// input will call into model from main
//


pub enum Surface {
    Water,
    Steel,
    PineWood,
    Plastic,
}


pub struct ModelParams {
    pub insolation: Insolation,
    pub surface: Surface,
}

fn main() {
    // let params = ModelParams {
    //    insolation:
    // };
}
