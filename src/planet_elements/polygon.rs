use std::error::Error;

use geojson::{Feature, Geometry, Value};

use super::{Arc, Point};

#[derive(Clone, Debug)]
pub struct Polygon {
    pub outline: Vec<Point>,
}

impl Polygon {
    pub fn new(outline: Vec<Point>) -> Polygon {
        Polygon { outline }
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Polygon, Box<dyn Error>> {
        let outline = vec
            .into_iter()
            .map(|point| Point::from_vec(point).unwrap())
            .collect();
        Ok(Polygon::new(outline))
    }

    pub fn contains(&self, point: &Point, not_inside: &Point) -> bool {
        let ray = Arc::new(point.clone(), not_inside.clone());
        let intersections = self.intersections(&ray).len();

        intersections % 2 == 1
    }

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        self.outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Arc::new(outline[0], outline[1]);
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
