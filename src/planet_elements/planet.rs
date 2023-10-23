use rayon::prelude::*;

use super::{coordinate::Coordinate, polygon::Polygon, raw_osm_data::RawOsmData};

pub struct Planet {
    pub polygons: Vec<Polygon>,
    pub points: Vec<Coordinate>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            points: Vec::new(),
        }
    }

    pub fn from_osm(path: &str) -> Self {
        let raw_osm_data = RawOsmData::from_path(path);
        raw_osm_data.to_planet()
    }

    pub fn is_on_land_ray(&self, point: &Coordinate) -> bool {
        // let north_pole = GeodeticCoordinate {
        //     lat: 90.0,
        //     lon: 0.0,
        // };
        let sixty_point = Coordinate::from_geodetic(-60.0, point.lon);
        // let check_north = self
        //     .polygons
        //     .par_iter()
        //     .any(|polygon| polygon.contains(point, &north_pole));
        let check_south = self
            .polygons
            .par_iter()
            .any(|polygon| polygon.contains(point, &sixty_point));
        // check_south == true || check_north == true
        check_south
    }

    pub fn to_json(&self) -> String {
        let polygon_jsons: Vec<String> = self
            .polygons
            .iter()
            .map(|polygon| polygon.to_json())
            .collect();

        let points_jsons: Vec<String> = self.points.iter().map(|point| point.to_json()).collect();

        vec![polygon_jsons.join("\n"), points_jsons.join("\n")].join("\n")
    }
}
