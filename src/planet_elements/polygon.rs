use geojson::{Feature, Geometry, Value};

use super::{coordinate::Coordinate, line::Line};

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon {
    pub outline: Vec<Coordinate>,
}

impl Polygon {
    pub fn new(outline: Vec<Coordinate>) -> Self {
        Self { outline }
    }

    pub fn contains(&self, point: &Coordinate, not_inside: &Coordinate) -> bool {
        let ray = Line {
            start: point.clone(),
            end: not_inside.clone(),
        };
        let intersections = self.intersections(&ray).len();

        intersections % 2 == 1
    }

    pub fn intersections(&self, line: &Line) -> Vec<Coordinate> {
        self.outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Line::new(outline[0], outline[1]);
                line.intersection(&outline)
            })
            .collect()
    }

    pub fn to_feature(&self) -> Feature {
        let polygon = self
            .outline
            .iter()
            .map(|&coordinate| vec![coordinate.lon, coordinate.lat])
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
