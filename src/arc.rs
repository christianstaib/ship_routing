use std::{error::Error, f64::consts::PI};

use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;
use rand::Rng;

use crate::{meters_to_radians, Point};

#[derive(Clone, Copy, PartialEq)]
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
        let north_pole = Point::from_coordinate(90.0, 0.0);

        let c1 = self.from.n_vector().cross(&self.to.n_vector());
        let c2 = self.from.n_vector().cross(&north_pole.n_vector());

        let mut sign = 1.0;
        if c1.cross(&c2).dot(&self.from.n_vector()) < 0.0 {
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
            &Point::from_geojson_vec(vec[0].clone()),
            &Point::from_geojson_vec(vec[1].clone()),
        ))
    }

    pub fn is_on_righthand_side(&self, point: &Point) -> bool {
        self.normal().dot(point.n_vector()) > 0.0
    }

    pub fn middle_random(&self) -> Point {
        let mut rng = rand::thread_rng();
        let f = rng.gen_range(0.0..1.0);
        Point::from_n_vector(&((self.from.n_vector() * (1.0 - f) + self.to.n_vector() * f) / 2.0))
    }

    pub fn middle(&self) -> Point {
        Point::from_n_vector(&((self.from.n_vector() + self.to.n_vector()) / 2.0))
    }

    pub fn from(&self) -> &Point {
        &self.from
    }

    pub fn to(&self) -> &Point {
        &self.to
    }

    /// Checks if other intersects self.
    ///
    /// Caveate: If the intersection point is near self.from it will not be returned. If it is near
    /// self.to it will be returned. This ensures that for a continoues path of arcs only one
    /// intersection is returned. So this functions is not symetrical. If you want to check for
    /// path, you need to ensure that for all arcs in the path you call arc.intersection(ray).
    pub fn intersection(&self, other: &Arc) -> Option<Point> {
        // check if both end or start on same point
        if self.from.is_approximately_equal(&other.from)
            || self.from.is_approximately_equal(&other.to)
        {
            return Some(self.from);
        } else if self.to.is_approximately_equal(&other.from)
            || self.to.is_approximately_equal(&other.to)
        {
            return Some(self.to);
        }

        // check if intersection of both great circles lies on both arcs
        let candidate = self.normal().cross(&other.normal()).normalize();
        if !candidate.x.is_nan() && !candidate.y.is_nan() && !candidate.z.is_nan() {
            let candidate = Point::from_n_vector(&candidate);
            if self.validate_intersection_candidate(&candidate)
                && other.validate_intersection_candidate(&candidate)
            {
                if !candidate.is_approximately_equal(self.from()) {
                    return Some(candidate);
                }
            }
            let candidate = candidate.antipode();
            if self.validate_intersection_candidate(&candidate)
                && other.validate_intersection_candidate(&candidate)
            {
                if !candidate.is_approximately_equal(self.from()) {
                    return Some(candidate);
                }
            }
        }

        None
    }

    pub fn intersects(&self, other: &Arc) -> bool {
        self.intersection(other).is_some()
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.from.n_vector().cross(&self.to.n_vector()).normalize()
    }

    fn from_normal(&self) -> Vector3<f64> {
        self.normal().cross(&self.from.n_vector()).normalize()
    }

    fn to_normal(&self) -> Vector3<f64> {
        self.normal().cross(&self.to.n_vector()).normalize()
    }

    pub fn collides(&self, point: &Point) -> bool {
        let summed_angle = Arc::new(&self.from(), point).central_angle()
            + Arc::new(point, &self.to()).central_angle();
        (summed_angle - self.central_angle()).abs() < meters_to_radians(1.0)
    }

    pub fn collides_arc(&self, arc: &Arc) -> bool {
        self.intersects(&arc)
            || self.collides(arc.from())
            || self.collides(arc.to())
            || arc.collides(self.from())
            || arc.collides(self.to())
    }

    fn validate_intersection_candidate(&self, point: &Point) -> bool {
        let a0 = point.n_vector().dot(&self.from_normal());
        let a1 = point.n_vector().dot(&self.to_normal());

        (a0 >= 0.0 && a1 <= 0.0)
            || (a0 >= 0.0 && point.is_approximately_equal(&self.from))
            || (a1 <= 0.0 && point.is_approximately_equal(&self.to))
    }

    pub fn central_angle(&self) -> f64 {
        let a = self.from.n_vector();
        let b = self.to.n_vector();
        a.angle(&b)
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        vec![self.from.to_geojson_vec(), self.to.to_geojson_vec()]
    }

    pub fn _make_good_line(&self) -> Vec<Arc> {
        let mut arcs = vec![self.clone()];
        while arcs[0].central_angle() > 0.05 {
            arcs = arcs
                .iter()
                .map(|arc| {
                    let middle = arc.middle();
                    vec![Arc::new(&arc.from(), &middle), Arc::new(&middle, &arc.to())]
                })
                .flatten()
                .collect();
        }
        arcs.retain(|arc| (arc.from().longitude() - arc.to().longitude()).abs() < 10.0);
        arcs
    }

    pub fn to_feature(&self) -> Feature {
        let point = Geometry::new(Value::LineString(vec![
            self.from.to_geojson_vec(),
            self.to.to_geojson_vec(),
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
        let from = Point::from_coordinate(90.0, 0.0);
        let to = Point::from_coordinate(0.0, 0.0);
        let arc = Arc::new(&from, &to);
        assert!((arc.central_angle() - (PI / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_intersection() {
        let outline_from = Point::from_coordinate(10.9602021, 119.7085977);
        let outline_to = Point::from_coordinate(10.9380527, 119.7102928);
        let outline = Arc::new(&outline_from, &outline_to);

        let ray_from = Point::from_coordinate(10.939165355971703, 119.71220924280686);
        let ray_to = Point::from_coordinate(11.42324706114331, 119.42008985034511);
        let ray = Arc::new(&ray_from, &ray_to);

        let intersect = ray.intersects(&outline);
        assert!(intersect)
    }

    #[test]
    fn test_edge_intersection() {
        let outline = vec![
            Point::from_coordinate(-1.0, 0.0),
            Point::from_coordinate(0.0, 0.0),
            Point::from_coordinate(1.0, 0.0),
        ];

        let ray = Arc::new(
            &Point::from_coordinate(0.0, 1.0),
            &Point::from_coordinate(0.0, -1.0),
        );

        let arc0 = Arc::new(&outline[0], &outline[1]);
        let arc1 = Arc::new(&outline[1], &outline[2]);

        assert!(arc0.intersects(&ray));
        assert!(!arc1.intersects(&ray));
    }
}
