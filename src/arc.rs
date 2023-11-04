use std::{error::Error, f64::consts::PI};

use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;

use crate::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arc {
    from: Point,
    to: Point,
}

impl Arc {
    pub fn new(start: &Point, end: &Point) -> Arc {
        Arc {
            from: start.clone(),
            to: end.clone(),
        }
    }

    // http://www.movable-type.co.uk/scripts/latlong-vectors.html
    // http://instantglobe.com/CRANES/GeoCoordTool.html
    pub fn initial_bearing(&self) -> f64 {
        let north_pole = Point::from_geodetic(90.0, 0.0);

        let c1 = self.from.vec().cross(&self.to.vec());
        let c2 = self.from.vec().cross(&north_pole.vec());

        let mut sign = 1.0;
        if c1.cross(&c2).dot(&self.from.vec()) < 0.0 {
            sign = -1.0;
        }

        let sin_theta = c1.cross(&c2).magnitude() * sign;
        let cos_theta = c1.dot(&c2);

        let mut theta = sin_theta.atan2(cos_theta);

        if theta < 0.0 {
            theta += 2.0 * PI;
        }

        //theta * 180.0 / PI
        theta
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Arc, Box<dyn Error>> {
        Ok(Arc::new(
            &Point::from_vec(vec[0].clone())?,
            &Point::from_vec(vec[1].clone())?,
        ))
    }

    pub fn is_on_righthand_side(&self, point: &Point) -> bool {
        self.normal().dot(point.vec()) >= 0.0
    }

    pub fn middle(&self) -> Point {
        Point::from_spherical(&((self.from.vec() + self.to.vec()) / 2.0))
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

    pub fn normal(&self) -> Vector3<f64> {
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

        (a0 >= 0.0 && a1 <= 0.0)
        // || ((a0 >= 0.0 || a1 <= 0.0) && (point.equals(&self.from) || point.equals(&self.to)))
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
        let arc = Arc::new(&from, &to);
        assert!((arc.central_angle() - (PI / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_intersection() {
        let outline_from = Point::from_geodetic(10.9602021, 119.7085977);
        let outline_to = Point::from_geodetic(10.9380527, 119.7102928);
        let outline = Arc::new(&outline_from, &outline_to);

        let ray_from = Point::from_geodetic(10.939165355971703, 119.71220924280686);
        let ray_to = Point::from_geodetic(11.42324706114331, 119.42008985034511);
        let ray = Arc::new(&ray_from, &ray_to);

        let intersect = ray.intersects(&outline);
        assert!(intersect)
    }
}
