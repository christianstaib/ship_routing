use geojson::{Feature, Geometry, Value};

use crate::{Arc, Point};

#[derive(Clone)]
pub struct Linestring {
    pub points: Vec<Point>,
}

impl Linestring {
    pub fn new(points: Vec<Point>) -> Linestring {
        let arcs: Vec<Arc> = points
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1])._make_good_line())
            .flatten()
            .collect();
        let mut points: Vec<Point> = arcs.iter().map(|arc| arc.from().clone()).collect();
        if !points.is_empty() {
            points.push(points.last().unwrap().clone());
        }

        Linestring { points }
    }

    pub fn to_feature(&self) -> Feature {
        let point = Geometry::new(Value::LineString(
            self.points.iter().map(|p| p.to_geojson_vec()).collect(),
        ));
        Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }
}
