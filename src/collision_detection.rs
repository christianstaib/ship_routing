use crate::{Arc, Point, Polygon};

pub trait CollisionDetection {
    fn add_polygon(&mut self, polygon: &Polygon);
    fn is_on_polygon(&self, point: &Point) -> bool;
    fn intersects_polygon(&self, arc: &Arc) -> bool;
}
