use crate::{Arc, Point, Polygon};

pub trait CollisionDetection {
    fn add_polygon(&mut self, polygon: &Polygon);
    fn is_on_polygon(&self, point: &Point) -> bool;
    fn intersects_polygon(&self, arc: &Arc) -> bool;
}

pub trait Collides<Rhs = Self> {
    // Required method
    fn collides(&self, rhs: &Rhs) -> bool;
}
