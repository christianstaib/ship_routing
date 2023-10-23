use nalgebra::Vector3;

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

    pub fn length(&self) -> f64 {
        let radius_of_earth: f64 = 6378.0;

        let start_lat = self.start.lat.to_radians();
        let start_lon = self.start.lon.to_radians();
        let end_lat = self.end.lat.to_radians();
        let end_lon = self.end.lon.to_radians();

        let length = radius_of_earth
            * (start_lat.sin() * end_lat.sin()
                + start_lat.cos() * end_lat.cos() * (end_lon - start_lon).cos())
            .acos();

        length
    }

    // https://blog.mbedded.ninja/mathematics/geometry/spherical-geometry/finding-the-intersection-of-two-arcs-that-lie-on-a-sphere/
    pub fn intersection(&self, other: &Line) -> bool {
        let p1 = self.start.vec;
        let p2 = self.end.vec;
        let p3 = other.start.vec;
        let p4 = other.end.vec;

        let n1 = p1.cross(&p2);
        let n2 = p3.cross(&p4);

        let l = n1.cross(&n2);

        if l.magnitude() == 0.0 {
            return false;
        }
        let i1 = l.normalize();
        let i2: Vector3<f64> = -1.0 * i1;

        let i1 = Coordinate::from_spherical(&i1);
        let i2 = Coordinate::from_spherical(&i2);

        if self.contains_point(&i1) && other.contains_point(&i1) {
            return true;
        } else if self.contains_point(&i2) && other.contains_point(&i2) {
            return true;
        }
        false
    }

    pub fn contains_point(&self, point: &Coordinate) -> bool {
        let arc_start = self.start.vec;
        let arc_end = self.end.vec;
        let point = point.vec;

        let total_angle = angle_between(&arc_start, &arc_end);
        let angle_sum = angle_between(&arc_start, &point) + angle_between(&point, &arc_end);

        (angle_sum - total_angle).abs() < 1e-3 // account for floating point inaccuracies
    }
}

pub fn angle_between(a: &Vector3<f64>, b: &Vector3<f64>) -> f64 {
    (a.dot(&b) / (a.magnitude() * b.magnitude())).acos()
}
