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

    pub fn lenght(&self) -> f64 {
        0.0
    }

    pub fn does_intersect(&self, other: &Line) -> bool {
        let p1 = SphericalCoordinate::from_node(&self.start);
        let p2 = SphericalCoordinate::from_node(&self.end);
        let p3 = SphericalCoordinate::from_node(&other.start);
        let p4 = SphericalCoordinate::from_node(&other.end);

        let n1 = p1.cross_product(&p2);
        let n2 = p3.cross_product(&p4);

        let l = n1.cross_product(&n2);

        if l.magnitude() == 0.0 {
            println!("magnitude is 0. return does not intersect");
            println!("{:?} {:?}", self, other);
            return false;
        }
        let i1 = l.normalize();
        let mut i2 = i1.clone();
        i2.divide_by_scalar(-1.0);

        (is_point_within_arc(&i1, &p1, &p2) && is_point_within_arc(&i1, &p3, &p4))
            || (is_point_within_arc(&i2, &p1, &p2) && is_point_within_arc(&i2, &p3, &p4))
    }

    pub fn contains_point(self, point: &GeodeticCoordinate) -> bool {
        let arc_start = SphericalCoordinate::from_node(&self.start);
        let arc_end = SphericalCoordinate::from_node(&self.end);
        let point = SphericalCoordinate::from_node(point);

        let total_angle = SphericalCoordinate::angle_between(&arc_start, &arc_end);
        let angle_sum = SphericalCoordinate::angle_between(&arc_start, &point)
            + SphericalCoordinate::angle_between(&point, &arc_end);

        (angle_sum - total_angle).abs() < 1e-6 // account for floating point inaccuracies
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
    (angle_sum - total_angle).abs() < 1e-6 // account for floating point inaccuracies
}
