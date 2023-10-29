use std::{error::Error, f64::consts::PI};

use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;

use super::Point;

#[derive(Clone, Copy, Debug)]
pub struct Arc {
    from: Point,
    to: Point,
}

impl Arc {
    pub fn new(start: Point, end: Point) -> Arc {
        Arc {
            from: start,
            to: end,
        }
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Arc, Box<dyn Error>> {
        Ok(Arc::new(
            Point::from_vec(vec[0].clone())?,
            Point::from_vec(vec[1].clone())?,
        ))
    }

    pub fn from(&self) -> &Point {
        &self.from
    }

    pub fn to(&self) -> &Point {
        &self.to
    }

    pub fn intersection(&self, other: &Arc) -> Option<Point> {
        // check if both end or start on same point
        if self.from.equals(&other.from) || self.from.equals(&other.to) {
            return Some(self.from);
        } else if self.to.equals(&other.from) || self.to.equals(&other.to) {
            return Some(self.to);
        }

        // check if both arcs are near enough to each other that they could intersect
        let summed_angle = self.central_angle() + other.central_angle();
        if summed_angle < PI {
            let summed_angle_cos = summed_angle.cos() - 0.0005;
            if self.from.vec().dot(&other.from.vec()) < summed_angle_cos
                || self.from.vec().dot(&other.to.vec()) < summed_angle_cos
                || self.to.vec().dot(&other.from.vec()) < summed_angle_cos
                || self.to.vec().dot(&other.to.vec()) < summed_angle_cos
            {
                return None;
            }
        }

        // check if intersection of both great circles lies on both arcs
        let candidate = self.normal().cross(&other.normal()).normalize();
        let candidate = Point::from_spherical(&candidate);
        if self.validate_intersection_candidate(&candidate)
            && other.validate_intersection_candidate(&candidate)
        {
            return Some(candidate);
        }
        let candidate = candidate.antipode();
        if self.validate_intersection_candidate(&candidate)
            && other.validate_intersection_candidate(&candidate)
        {
            return Some(candidate);
        }

        None
    }

    pub fn intersects(&self, other: &Arc) -> bool {
        self.intersection(other).is_some()
    }

    fn normal(&self) -> Vector3<f64> {
        self.from.vec().cross(&self.to.vec()).normalize()
    }

    fn from_normal(&self) -> Vector3<f64> {
        self.normal().cross(&self.from.vec()).normalize()
    }

    fn to_normal(&self) -> Vector3<f64> {
        self.normal().cross(&self.to.vec()).normalize()
    }

    fn validate_intersection_candidate(&self, point: &Point) -> bool {
        let a0 = point.vec().dot(&self.from_normal());
        let a1 = point.vec().dot(&self.to_normal());

        (a0 > 0.0 && a1 < 0.0)
            || (a0 >= 0.0 && a1 <= 0.0 && (point.equals(&self.from) || point.equals(&self.to)))
    }

    pub fn central_angle(&self) -> f64 {
        let a = self.from.vec();
        let b = self.to.vec();
        a.angle(&b)
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        vec![self.from.to_vec(), self.to.to_vec()]
    }

    pub fn to_feature(&self) -> Feature {
        let point = Geometry::new(Value::LineString(vec![
            self.from.to_vec(),
            self.to.to_vec(),
        ]));
        Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{Arc, Point};

    #[test]
    fn test_central_angle() {
        let from = Point::from_geodetic(90.0, 0.0);
        let to = Point::from_geodetic(0.0, 0.0);
        let arc = Arc::new(from, to);
        assert!((arc.central_angle() - (PI / 2.0)).abs() < 1e-10);
    }
}
