use indicatif::ProgressIterator;

use super::{coordinate::GeodeticCoordinate, line::Line, rectangle::Rectangle};

#[derive(Clone, Debug)]
pub struct Polygon {
    outline: Vec<GeodeticCoordinate>,
    bounding_box: Rectangle,
}

impl Polygon {
    pub fn new(outline: Vec<GeodeticCoordinate>) -> Self {
        let bounding_box = Rectangle::new(&outline);
        Self {
            outline,
            bounding_box,
        }
    }

    pub fn contains(&self, point: &GeodeticCoordinate) -> bool {
        let north_pole = GeodeticCoordinate {
            lat: 90.0,
            lon: 0.0,
        };
        let intersections = self
            .outline
            .windows(2)
            .filter(|outline| {
                let min_lon_outline = f64::min(outline[0].lon, outline[1].lon);
                let max_lon_outline = f64::max(outline[0].lon, outline[1].lon);
                min_lon_outline <= point.lon && point.lon <= max_lon_outline
            })
            .map(|outline| {
                let outline = Line {
                    start: outline[0],
                    end: outline[1],
                };
                let ray = Line {
                    start: point.clone(),
                    end: north_pole,
                };
                ray.does_intersect(&outline)
            })
            .filter(|&x| x == true)
            .count();
        intersections % 2 == 1
    }

    pub fn contains_bounding_box(&self, point: &GeodeticCoordinate) -> bool {
        self.bounding_box.contains(point)
    }
}
