use std::{f64::consts::PI, vec};

use geojson::{Feature, Geometry, Value};

use super::{Arc, Point};

pub struct EverythingPolygon {}

impl EverythingPolygon {
    pub fn new() -> EverythingPolygon {
        EverythingPolygon {}
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub outline: Vec<Point>,
    pub inside_point: Point,
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

    pub fn arcs(&self) -> Vec<Arc> {
        self.outline
            .windows(2)
            .map(|outline| Arc::new(&outline[0], &outline[1]))
            .collect()
    }

    pub fn get_inside_point(&self) -> Point {
        let outline = Arc::new(&self.outline[0], &self.outline[1]);
        let middle = outline.middle();
        let destination =
            Point::destination_point(&middle, outline.initial_bearing() + (PI / 2.0), -0.01);
        let md = Arc::new(&middle, &destination);
        let mut intersections = self.intersections(&md);
        intersections.sort_by(|&a, &b| {
            let a_dist = Arc::new(&middle, &a).central_angle();
            let b_dist = Arc::new(&middle, &b).central_angle();
            a_dist.partial_cmp(&b_dist).unwrap()
        });

        // make sure middle is in list
        if let Some(first) = intersections.first() {
            if !middle.is_approximately_equal(first) {
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

    pub fn from_geojson_vec(vec: Vec<Vec<f64>>) -> Polygon {
        let outline = vec
            .into_iter()
            .map(|point| Point::from_geojson_vec(point))
            .collect();
        Polygon::new(outline)
    }

    pub fn to_geojson_vec(&self) -> Vec<Vec<f64>> {
        self.outline
            .iter()
            .map(|&coordinate| coordinate.to_geojson_vec())
            .collect()
    }

    pub fn to_feature(&self) -> Feature {
        let polygon = self.to_geojson_vec();
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
