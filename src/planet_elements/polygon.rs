use std::error::Error;

use geojson::{Feature, Geometry, Value};

use super::{Arc, Point};

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    pub outline: Vec<Point>,
}

impl Polygon {
    pub fn new(outline: Vec<Point>) -> Polygon {
        Polygon { outline }
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Polygon, Box<dyn Error>> {
        let outline = vec
            .into_iter()
            .map(|point| Point::from_vec(point).unwrap())
            .collect();
        Ok(Polygon::new(outline))
    }

    pub fn fast_intersections(&self, point: &Point) -> Vec<Point> {
        let north_pole = Point::from_geodetic(90.0, 0.0);
        let ray = Arc {
            start: point.clone(),
            end: north_pole.clone(),
        };
        self.outline
            .windows(2)
            .map(|points| Arc::new(points[0], points[1]))
            .filter(|arc| {
                let lon_min = arc.start.lon.min(arc.end.lon);
                let lon_max = arc.start.lon.max(arc.end.lon);
                lon_min <= point.lon && point.lon <= lon_max
            })
            .filter_map(|arc| ray.intersection(&arc))
            .collect()
    }

    pub fn fast_contains(&self, point: &Point) -> bool {
        let north_pole = Point::from_geodetic(90.0, 0.0);
        let ray = Arc {
            start: point.clone(),
            end: north_pole.clone(),
        };
        let intersections = self
            .outline
            .windows(2)
            .map(|points| Arc::new(points[0], points[1]))
            .filter(|arc| {
                let lon_min = arc.start.lon.min(arc.end.lon);
                let lon_max = arc.start.lon.max(arc.end.lon);
                lon_min <= point.lon && point.lon <= lon_max
            })
            .filter(|arc| ray.intersects(arc))
            .count();

        // if intersections % 2 == 1 {
        //     println!("collision with {:?}", self.outline[0]);
        // }

        intersections % 2 == 1
    }

    pub fn contains(&self, point: &Point, not_inside: &Point) -> bool {
        let ray = Arc {
            start: point.clone(),
            end: not_inside.clone(),
        };
        let intersections = self.intersections(&ray).len();

        intersections % 2 == 1
    }

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        self.outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Arc::new(outline[0], outline[1]);
                line.intersection(&outline)
            })
            .collect()
    }

    pub fn to_feature(&self) -> Feature {
        let polygon = self
            .outline
            .iter()
            .map(|&coordinate| vec![coordinate.lon, coordinate.lat])
            .collect();

        let polygon = Geometry::new(Value::Polygon(vec![polygon]));
        Feature {
            bbox: None,
            geometry: Some(polygon),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }

    pub fn to_json(&self) -> String {
        self.to_feature().to_string()
    }
}
