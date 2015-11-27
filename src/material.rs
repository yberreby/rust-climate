use std::fmt;
/**
 * Sources:
 * // https://corona-renderer.com/forum/index.php?topic=2359.0 albedo
 * // http://www.engineeringtoolbox.com/emissivity-coefficients-d_447.html emissivity
**/
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub name: &'static str,
    pub emissivity: f64,
    pub albedo: f64,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} (coefficient d'émissivité {}, albedo {})",
               self.name,
               self.emissivity,
               self.albedo)
    }
}

pub const STAINLESS_STEEL: Material = Material {
    name: "acier inoxydable",
    emissivity: 0.6, // Stainless Steel, type 301	0.54 - 0.63
    albedo: 0.24,
};

pub const CONCRETE: Material = Material {
    name: "béton",
    emissivity: 0.85,
    albedo: 0.3,
};

// http://www.engineeringtoolbox.com/radiation-heat-emissivity-d_432.html emissivity OF GRASS
pub const GREEN_GRASS: Material = Material {
    name: "herbe verte",
    emissivity: 0.98,
    albedo: 0.25,
};

pub const SAND: Material = Material {
    name: "sable",
    emissivity: 0.76,
    albedo: 0.35,
};
