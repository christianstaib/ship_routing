use geojson::{Feature, Geometry, Value};

use crate::EPSILON;

use super::coordinate::Coordinate;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Line {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Self { start, end }
    }

    // https://blog.mbedded.ninja/mathematics/geometry/spherical-geometry/finding-the-intersection-of-two-arcs-that-lie-on-a-sphere/
    pub fn intersection(&self, other: &Line) -> Option<Coordinate> {
        let normal1 = (self.start.vec).cross(&self.end.vec);
        let normal2 = (other.start.vec).cross(&other.end.vec);

        let line = normal1.cross(&normal2);
        if line.magnitude() == 0.0 {
            return None;
        }
        let line = line.normalize();

        let intersection1 = line;
        let intersection2 = -1.0 * line;

        let intersection1 = Coordinate::from_spherical(&intersection1);
        let intersection2 = Coordinate::from_spherical(&intersection2);

        if self.contains_point(&intersection1) && other.contains_point(&intersection1) {
            return Some(intersection1);
        } else if self.contains_point(&intersection2) && other.contains_point(&intersection2) {
            return Some(intersection2);
        }
        None
    }

    pub fn contains_point(&self, point: &Coordinate) -> bool {
        let start_to_point = Line::new(self.start, *point);
        let point_to_end = Line::new(*point, self.end);

        let true_angle = self.central_angle();
        let angled_sum = start_to_point.central_angle() + point_to_end.central_angle();

        (angled_sum - true_angle).abs() < EPSILON
    }

    pub fn central_angle(&self) -> f64 {
        let a = self.start.vec;
        let b = self.end.vec;
        a.angle(&b)
    }

    pub fn to_feature(&self) -> Feature {
        let start: Vec<f64> = vec![self.start.lon, self.start.lat];
        let end: Vec<f64> = vec![self.end.lon, self.end.lat];
        let point = Geometry::new(Value::LineString(vec![start, end]));
        Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }

    pub fn to_json(&self) -> String {
        self.to_feature().to_string()
    }
}
