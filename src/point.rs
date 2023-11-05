use std::{error::Error, f64::consts::PI, fmt};

use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;
use rand::Rng;

use crate::Arc;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    vec: Vector3<f64>,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(lat:{}, lon::{})", self.lat(), self.lon())
    }
}

impl Point {
    pub fn from_geodetic(lat: f64, lon: f64) -> Point {
        assert!(-90.0 <= lat && lat <= 90.0, "illegal lat: {}", lat);
        assert!(-180.0 <= lon && lon <= 180.0, "illegal lon: {}", lon);

        let lat_rad = lat.to_radians();
        let lon_rad = lon.to_radians();
        let vec = Vector3::new(
            lat_rad.cos() * lon_rad.cos(),
            lat_rad.cos() * lon_rad.sin(),
            lat_rad.sin(),
        );

        Point { vec }
    }

    pub fn from_spherical(vec: &Vector3<f64>) -> Point {
        let point = Point { vec: vec.clone() };

        let lat = point.lat();
        let lon = point.lon();
        assert!(-90.0 <= lat && lat <= 90.0, "illegal lat: {}", lat);
        assert!(-180.0 <= lon && lon <= 180.0, "illegal lon: {}", lon);

        point
    }

    pub fn random() -> Point {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen_range(-1.0..1.0);
        let lat_rad: f64 = y.asin();
        let lat: f64 = lat_rad.to_degrees();
        let lon: f64 = rng.gen_range(-180.0..180.0);
        Point::from_geodetic(lat, lon)
    }

    // http://www.movable-type.co.uk/scripts/latlong-vectors.html
    pub fn destination_point(start: &Point, bearing: f64, distance: f64) -> Point {
        let n = Point::from_geodetic(90.0, 0.0);
        let de = n.vec().cross(start.vec());
        let dn = start.vec().cross(&de);
        let d = dn * bearing.cos() + de * bearing.sin();
        let b = start.vec() * distance.cos() + d * distance.sin();
        Point::from_spherical(&b)
    }

    pub fn lat(&self) -> f64 {
        self.vec
            .z
            .atan2((self.vec.x.powi(2) + self.vec.y.powi(2)).sqrt())
            .to_degrees()
    }

    pub fn lon(&self) -> f64 {
        self.vec
            .y
            .to_radians()
            .atan2(self.vec.x.to_radians())
            .to_degrees()
    }

    pub fn vec(&self) -> &Vector3<f64> {
        &self.vec
    }

    pub fn equals(&self, other: &Point) -> bool {
        Arc::new(self, other).central_angle() < meters_to_radians(0.1)
    }

    pub fn antipode(&self) -> Point {
        Point::from_spherical(&-self.vec)
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.lon(), self.lat()]
    }

    pub fn from_vec(vec: Vec<f64>) -> Result<Point, Box<dyn Error>> {
        Ok(Point::from_geodetic(vec[1], vec[0]))
    }

    pub fn to_feature(&self) -> Feature {
        let point: Vec<f64> = vec![self.lon(), self.lat()];
        let point = Geometry::new(Value::Point(point));
        Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }
}

fn meters_to_radians(meters: f64) -> f64 {
    const EARTH_RADIUS_METERS: usize = 6_378_160;
    2.0 * PI / EARTH_RADIUS_METERS as f64 * meters
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn test_point_convertion() {
        for _ in 0..100 {
            let point = Point::random();
            assert!(point.equals(&Point::from_spherical(point.vec())));
        }
    }
}
