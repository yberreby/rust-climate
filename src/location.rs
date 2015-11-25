use std::fmt;

pub struct Location {
    pub name: &'static str,
    pub coords: GpsCoordinates,
}

impl Location {
    pub fn new(name: &'static str, lat: f64, long: f64) -> Location {
        assert!(-90.0 <= lat && lat <= 90.0);
        assert!(-180.0 <= long && long <= 180.0);

        Location {
            name: name,
            coords: GpsCoordinates {
                lat: lat,
                long: long,
            },
        }
    }

}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} (latitude : {:.4}, longitude : {:.4})",
               self.name,
               self.coords.lat,
               self.coords.long)
    }
}

pub struct GpsCoordinates {
    pub lat: f64,
    pub long: f64,
}


// coordinates from Google

// north-west
pub const SEATTLE: Location = Location {
    name: "Seattle",
    coords: GpsCoordinates {
        lat: 47.6097,
        long: -122.3331,
    },
};
// north-east
pub const BOSTON: Location = Location {
    name: "Boston",
    coords: GpsCoordinates {
        lat: 42.3601,
        long: -71.0589,
    },
};

// south-west
pub const LOS_ANGELES: Location = Location {
    name: "Los Angeles",
    coords: GpsCoordinates {
        lat: 34.0500,
        long: -118.2500,
    },
};

// south-east
pub const MIAMI: Location = Location {
    name: "Miami",
    coords: GpsCoordinates {
        lat: 25.7753,
        long: -80.2089,
    },
};

// center
pub const DENVER: Location = Location {
    name: "Denver",
    coords: GpsCoordinates {
        lat: 39.7392,
        long: -104.9903,
    },
};
