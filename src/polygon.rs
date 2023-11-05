use std::{error::Error, f64::consts::PI, vec};

use geojson::{Feature, Geometry, Value};

use super::{Arc, Point};

pub trait SolidShape {
    fn contains(&self, point: &Point) -> bool;
    fn intersects(&self, arc: &Arc) -> bool;
}

pub struct EverythingPolygon;
impl SolidShape for EverythingPolygon {
    fn contains(&self, _point: &Point) -> bool {
        true
    }

    fn intersects(&self, _arc: &Arc) -> bool {
        false
    }
}

pub struct NothingPolygon;
impl SolidShape for NothingPolygon {
    fn contains(&self, _point: &Point) -> bool {
        false
    }

    fn intersects(&self, _arc: &Arc) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub outline: Vec<Point>,
    pub inside_point: Point,
}
impl SolidShape for Polygon {
    fn contains(&self, point: &Point) -> bool {
        let ray = Arc::new(point, &self.inside_point);
        let intersections = self.intersections(&ray).len();
        intersections % 2 == 0
    }

    fn intersects(&self, arc: &Arc) -> bool {
        !self.intersections(arc).is_empty()
    }
}

impl Polygon {
    pub fn new(outline: Vec<Point>) -> Polygon {
        let mut polygon = Polygon {
            outline,
            inside_point: Point::random(),
        };

        polygon.inside_point = polygon.get_inside_point();
        polygon
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Polygon, Box<dyn Error>> {
        let outline = vec
            .into_iter()
            .map(|point| Point::from_vec(point).unwrap())
            .collect();
        Ok(Polygon::new(outline))
    }

    pub fn get_inside_point(&self) -> Point {
        let ab = Arc::new(&self.outline[0], &self.outline[1]);
        let middle = ab.middle();
        let destination =
            Point::destination_point(&middle, ab.initial_bearing() + (PI / 2.0), -0.01);
        let md = Arc::new(&middle, &destination);
        let mut intersections = self.intersections(&md);
        intersections.sort_by(|&a, &b| {
            let a_dist = Arc::new(&middle, &a).central_angle();
            let b_dist = Arc::new(&middle, &b).central_angle();
            a_dist.partial_cmp(&b_dist).unwrap()
        });

        // make sure middle is in list
        if let Some(first) = intersections.first() {
            if !middle.equals(first) {
                intersections.insert(0, middle);
            }
        } else {
            intersections.insert(0, middle);
        }

        intersections.push(destination);

        Arc::new(&intersections[0], &intersections[1]).middle()
    }

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        self.outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Arc::new(&outline[0], &outline[1]);
                line.intersection(&outline)
            })
            .collect()
    }

    pub fn to_feature(&self) -> Feature {
        let polygon = self
            .outline
            .iter()
            .map(|&coordinate| vec![coordinate.lon(), coordinate.lat()])
            .collect();

        let polygon = Geometry::new(Value::Polygon(vec![polygon]));
        Feature {
            bbox: None,
            geometry: Some(polygon),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }

    pub fn to_json(&self) -> String {
        self.to_feature().to_string()
    }
}
