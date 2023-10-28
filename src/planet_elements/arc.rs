use std::error::Error;

use geojson::{Feature, Geometry, Value};

use super::Point;

#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub start: Point,
    pub end: Point,
}

impl Arc {
    pub fn new(start: Point, end: Point) -> Arc {
        Arc { start, end }
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Arc, Box<dyn Error>> {
        Ok(Arc::new(
            Point::from_vec(vec[0].clone())?,
            Point::from_vec(vec[1].clone())?,
        ))
    }

    // https://blog.mbedded.ninja/mathematics/geometry/spherical-geometry/finding-the-intersection-of-two-arcs-that-lie-on-a-sphere/
    pub fn intersection(&self, other: &Arc) -> Option<Point> {
        let normal1 = (self.start.vec).cross(&self.end.vec);
        let normal2 = (other.start.vec).cross(&other.end.vec);

        let line = normal1.cross(&normal2);
        if line.magnitude() == 0.0 {
            return None;
        }
        let line = line.normalize();

        let intersection1 = line;
        let intersection2 = -1.0 * line;

        let intersection1 = Point::from_spherical(&intersection1);
        let intersection2 = Point::from_spherical(&intersection2);

        if self.contains_point(&intersection1) && other.contains_point(&intersection1) {
            return Some(intersection1);
        } else if self.contains_point(&intersection2) && other.contains_point(&intersection2) {
            return Some(intersection2);
        }
        None
    }

    pub fn intersects(&self, other: &Arc) -> bool {
        self.intersection(other).is_some()
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        let start_to_point = Arc::new(self.start, *point);
        let point_to_end = Arc::new(*point, self.end);

        let true_angle = self.central_angle();
        let angled_sum = start_to_point.central_angle() + point_to_end.central_angle();

        (angled_sum - true_angle).abs() < 0.0005
    }

    pub fn central_angle(&self) -> f64 {
        let a = self.start.vec;
        let b = self.end.vec;
        a.angle(&b)
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        vec![self.start.to_vec(), self.end.to_vec()]
    }

    pub fn to_feature(&self) -> Feature {
        let point = Geometry::new(Value::LineString(vec![
            self.start.to_vec(),
            self.end.to_vec(),
        ]));
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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{Arc, Point};

    #[test]
    fn test_central_angle() {
        let start = Point::from_geodetic(90.0, 0.0);
        let end = Point::from_geodetic(0.0, 0.0);
        let arc = Arc::new(start, end);
        assert!((arc.central_angle() - (PI / 2.0)).abs() < 1e-10);
    }
}
