use super::coordinate::GeodeticCoordinate;

#[derive(Clone, Debug)]
pub struct Rectangle {
    min_point: GeodeticCoordinate,
    max_point: GeodeticCoordinate,
}

impl Rectangle {
    pub fn new(outline: &Vec<GeodeticCoordinate>) -> Self {
        let min_lat = outline
            .iter()
            .min_by(|coordinate1, coordinate2| coordinate1.lat.total_cmp(&coordinate2.lat))
            .unwrap()
            .lat;
        let min_lon = outline
            .iter()
            .min_by(|coordinate1, coordinate2| coordinate1.lon.total_cmp(&coordinate2.lat))
            .unwrap()
            .lon;
        let max_lat = outline
            .iter()
            .max_by(|coordinate1, coordinate2| coordinate1.lat.total_cmp(&coordinate2.lat))
            .unwrap()
            .lat;
        let max_lon = outline
            .iter()
            .max_by(|coordinate1, coordinate2| coordinate1.lon.total_cmp(&coordinate2.lon))
            .unwrap()
            .lon;
        let min_point = GeodeticCoordinate {
            lon: min_lon,
            lat: min_lat,
        };
        let max_point = GeodeticCoordinate {
            lon: max_lon,
            lat: max_lat,
        };
        Self {
            min_point,
            max_point,
        }
    }

    pub fn contains(&self, point: &GeodeticCoordinate) -> bool {
        self.min_point.lat <= point.lat
            && point.lat <= self.max_point.lat
            && self.min_point.lon <= point.lon
            && point.lon <= self.max_point.lon
    }
}
