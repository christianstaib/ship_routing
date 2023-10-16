use rayon::prelude::ParallelBridge;
use rayon::prelude::*;

use crate::planet::Planet;

use super::{coordinate::GeodeticCoordinate, polygon::Polygon};

pub struct NewPlanet {
    pub land_mass: Vec<Polygon>,
}

impl NewPlanet {
    pub fn from_planet(planet: &Planet) -> Self {
        let land_mass: Vec<Polygon> = planet
            .coastlines
            .iter()
            .cloned()
            .map(|outline| Polygon::new(outline))
            .collect();

        Self { land_mass }
    }

    pub fn is_on_land(&self, point: &GeodeticCoordinate) -> bool {
        self.land_mass
            .par_iter()
            //.filter(|polygon| polygon.contains_bounding_box(point))
            .any(|polygon| polygon.contains(point))
    }
}
