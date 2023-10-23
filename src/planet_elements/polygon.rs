use std::f64::consts::PI;

use geojson::{Feature, Geometry, Value};

use super::{
    coordinate::{subtended_angle, GeodeticCoordinate},
    line::Line,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    pub outline: Vec<GeodeticCoordinate>,
}

impl Polygon {
    pub fn new(outline: Vec<GeodeticCoordinate>) -> Self {
        Self { outline }
    }

    pub fn contains(&self, point: &GeodeticCoordinate, not_inside: &GeodeticCoordinate) -> bool {
        let intersections = self
            .outline
            .windows(2)
            .map(|line| Line {
                start: line[0],
                end: line[1],
            })
            .filter(|line| {
                // speed up calculation. Works only if north pole is on water and
                // if no edge goes from -180 to 180
                let min_lon_outline = f64::min(line.start.lon, line.end.lon);
                let max_lon_outline = f64::max(line.start.lon, line.end.lon);
                min_lon_outline <= point.lon && point.lon <= max_lon_outline
            })
            .map(|line| {
                let ray = Line {
                    start: point.clone(),
                    end: not_inside.clone(),
                };
                ray.intersection(&line)
            })
            .filter(|&x| x.is_some())
            .count();
        intersections % 2 == 1
    }

    pub fn winding_numer(&self, point: &GeodeticCoordinate) -> f64 {
        self.outline
            .windows(2)
            .map(|l| subtended_angle(point, &l[0], &l[1]))
            .sum()
    }

    pub fn contains_winding(&self, point: &GeodeticCoordinate) -> bool {
        let winding_number = self.winding_numer(point);
        let winding_number = winding_number % (2.0 * PI);
        let winding_number = winding_number.abs();
        println!("{}", winding_number);
        winding_number >= 0.000_000_1
    }

    pub fn to_json(&self) -> String {
        let polygon = self
            .outline
            .iter()
            .map(|&coordinate| vec![coordinate.lon, coordinate.lat])
            .collect();

        let polygon = Geometry::new(Value::Polygon(vec![polygon]));
        let geometry = Feature {
            bbox: None,
            geometry: Some(polygon),
            id: None,
            properties: None,
            foreign_members: None,
        };
        geometry.to_string()
    }
}
