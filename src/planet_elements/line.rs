use nalgebra::Vector3;

use super::coordinate::{GeodeticCoordinate, SphericalCoordinate};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: GeodeticCoordinate,
    pub end: GeodeticCoordinate,
}

impl Line {
    pub fn new(start: GeodeticCoordinate, end: GeodeticCoordinate) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f64 {
        let radius_of_earth: f64 = 6378.0;

        let start_lat = (self.start.lat) * std::f64::consts::PI / 180.0;
        let start_lon = (self.start.lon) * std::f64::consts::PI / 180.0;
        let end_lat = (self.end.lat) * std::f64::consts::PI / 180.0;
        let end_lon = (self.end.lon) * std::f64::consts::PI / 180.0;

        let length = radius_of_earth
            * (start_lat.sin() * end_lat.sin()
                + start_lat.cos() * end_lat.cos() * (end_lon - start_lon).cos())
            .acos();

        length
    }

    // https://blog.mbedded.ninja/mathematics/geometry/spherical-geometry/finding-the-intersection-of-two-arcs-that-lie-on-a-sphere/
    pub fn intersection(&self, other: &Line) -> bool {
        let p1 = self.start.to_vector3();
        let p2 = self.end.to_vector3();
        let p3 = other.start.to_vector3();
        let p4 = other.end.to_vector3();

        let n1 = p1.cross(&p2);
        let n2 = p3.cross(&p4);

        let l = n1.cross(&n2);

        if l.magnitude() == 0.0 {
            // println!("magnitude is 0. return does not intersect");
            // println!("{:?} {:?}", self, other);
            return false;
        }
        let i1 = l.normalize();
        let i2: Vector3<f64> = -1.0 * i1;

        if is_point_within_arc(&i1, &p1, &p2) && is_point_within_arc(&i1, &p3, &p4) {
            return true;
        } else if is_point_within_arc(&i2, &p1, &p2) && is_point_within_arc(&i2, &p3, &p4) {
            return true;
        }
        false
    }

    pub fn contains_point(&self, point: &GeodeticCoordinate) -> bool {
        let arc_start = self.start.to_vector3();
        let arc_end = self.end.to_vector3();
        let point = point.to_vector3();

        let total_angle = angle_between(&arc_start, &arc_end);
        let angle_sum = angle_between(&arc_start, &point) + angle_between(&point, &arc_end);

        (angle_sum - total_angle).abs() < 1e-9 // account for floating point inaccuracies
    }
}

pub fn angle_between(a: &Vector3<f64>, b: &Vector3<f64>) -> f64 {
    (a.dot(&b) / (a.magnitude() * b.magnitude())).acos()
}

pub fn is_point_within_arc(
    point: &Vector3<f64>,
    arc_start: &Vector3<f64>,
    arc_end: &Vector3<f64>,
) -> bool {
    let total_angle = angle_between(arc_start, arc_end);
    let angle_sum = angle_between(arc_start, point) + angle_between(point, arc_end);
    (angle_sum - total_angle).abs() < 1e-9 // account for floating point inaccuracies
}
