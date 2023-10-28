use rand::{rngs::ThreadRng, Rng};

use crate::planet_elements::Point;

pub const EPS: f64 = 0.001;

pub struct PointGenerator {
    rng: ThreadRng,
}

impl PointGenerator {
    pub fn new() -> Self {
        PointGenerator {
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for PointGenerator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let y: f64 = self.rng.gen_range(-1.0..1.0);
        let lat_rad: f64 = y.asin();
        let lat: f64 = lat_rad.to_degrees();
        let lon: f64 = self.rng.gen_range(-180.0..180.0);
        let point = Point::from_geodetic(lat, lon);
        Some(point)
    }
}