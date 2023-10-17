use super::{coordinate::GeodeticCoordinate, line::Line};

#[derive(Clone, Debug)]
pub struct Polygon {
    outline: Vec<GeodeticCoordinate>,
}

impl Polygon {
    pub fn new(outline: Vec<GeodeticCoordinate>) -> Self {
        Self { outline }
    }

    pub fn contains(&self, point: &GeodeticCoordinate) -> bool {
        let north_pole = GeodeticCoordinate {
            lat: 90.0,
            lon: 0.0,
        };
        let intersections = self
            .outline
            .windows(2)
            .map(|line| Line {
                start: line[0],
                end: line[1],
            })
            .filter(|line| {
                // speed up calculation. Works only if north pole is on water and
                // if no edge goes from -180 to 180
                let min_lon_outline = f64::min(line.start.lon, line.end.lon);
                let max_lon_outline = f64::max(line.start.lon, line.end.lon);
                min_lon_outline <= point.lon && point.lon <= max_lon_outline
            })
            .map(|line| {
                let ray = Line {
                    start: point.clone(),
                    end: north_pole,
                };
                ray.intersection(&line)
            })
            .filter(|&x| x.is_some())
            .count();
        intersections % 2 == 1
    }
}
