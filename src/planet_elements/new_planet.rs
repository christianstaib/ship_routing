use rayon::prelude::*;

use crate::planet::Planet;

use super::{coordinate::GeodeticCoordinate, polygon::Polygon};

pub struct NewPlanet {
    pub land_mass: Vec<Polygon>,
}

impl NewPlanet {
    pub fn from_planet(planet: &Planet) -> Self {
        let mut coastlines = planet.coastlines.clone();
        // sort so that longest polygon is first, to make is_on_land faster
        coastlines.sort_unstable_by_key(|coastline| -1 * coastline.len() as isize);

        let land_mass: Vec<Polygon> = coastlines
            .iter()
            .cloned()
            .map(|outline| Polygon::new(outline))
            .collect();

        Self { land_mass }
    }

    pub fn is_on_land_ray(&self, point: &GeodeticCoordinate) -> bool {
        self.land_mass
            .par_iter()
            .any(|polygon| polygon.contains(point))
    }

    pub fn is_on_land_winding(&self, point: &GeodeticCoordinate) -> bool {
        self.land_mass
            .par_iter()
            .any(|polygon| polygon.contains_winding(point))
    }
}
