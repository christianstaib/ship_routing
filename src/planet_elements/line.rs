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
    pub fn intersection(&self, other: &Line) -> Option<SphericalCoordinate> {
        let p1 = SphericalCoordinate::from_node(&self.start);
        let p2 = SphericalCoordinate::from_node(&self.end);
        let p3 = SphericalCoordinate::from_node(&other.start);
        let p4 = SphericalCoordinate::from_node(&other.end);

        let n1 = p1.cross_product(&p2);
        let n2 = p3.cross_product(&p4);

        let l = n1.cross_product(&n2);

        if l.magnitude() == 0.0 {
            // println!("magnitude is 0. return does not intersect");
            // println!("{:?} {:?}", self, other);
            return None;
        }
        let i1 = l.normalize();
        let mut i2 = i1.clone();
        i2.divide_by_scalar(-1.0);

        if is_point_within_arc(&i1, &p1, &p2) && is_point_within_arc(&i1, &p3, &p4) {
            return Some(i1);
        } else if is_point_within_arc(&i2, &p1, &p2) && is_point_within_arc(&i2, &p3, &p4) {
            return Some(i2);
        }
        return None;
    }

    pub fn contains_point(self, point: &GeodeticCoordinate) -> bool {
        let arc_start = SphericalCoordinate::from_node(&self.start);
        let arc_end = SphericalCoordinate::from_node(&self.end);
        let point = SphericalCoordinate::from_node(point);

        let total_angle = SphericalCoordinate::angle_between(&arc_start, &arc_end);
        let angle_sum = SphericalCoordinate::angle_between(&arc_start, &point)
            + SphericalCoordinate::angle_between(&point, &arc_end);

        (angle_sum - total_angle).abs() < 1e-9 // account for floating point inaccuracies
    }
}

pub fn is_point_within_arc(
    point: &SphericalCoordinate,
    arc_start: &SphericalCoordinate,
    arc_end: &SphericalCoordinate,
) -> bool {
    let total_angle = SphericalCoordinate::angle_between(arc_start, arc_end);
    let angle_sum = SphericalCoordinate::angle_between(arc_start, point)
        + SphericalCoordinate::angle_between(point, arc_end);
    (angle_sum - total_angle).abs() < 1e-9 // account for floating point inaccuracies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let start = GeodeticCoordinate { lat: 0.0, lon: 0.0 };
        let end = GeodeticCoordinate { lat: 0.0, lon: 1.0 };
        let line = Line::new(start, end);

        let expected_length = 111.2;

        let actual_length = line.length();

        let tolerance_in_percent = 0.01;

        let difference_in_percent = (actual_length - expected_length).abs() / expected_length;
        assert!(
            difference_in_percent < tolerance_in_percent,
            "expected: {}, actual: {}, difference: {}%",
            expected_length,
            actual_length,
            difference_in_percent * 100.0
        );
    }
}
