use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
    pub vec: Vector3<f64>,
}

impl Coordinate {
    pub fn from_geodetic(lat: f64, lon: f64) -> Self {
        let lat_rad = lat.to_radians();
        let lon_rad = lon.to_radians();

        let vec = Vector3::new(
            lat_rad.cos() * lon_rad.cos(),
            lat_rad.cos() * lon_rad.sin(),
            lat_rad.sin(),
        );

        Self { lat, lon, vec }
    }

    pub fn from_spherical(vec: &Vector3<f64>) -> Self {
        let lat = vec.z.asin().to_degrees();
        let lon = vec.y.to_radians().atan2(vec.x.to_radians()).to_degrees();
        let vec = vec.clone();

        Self { lat, lon, vec }
    }

    pub fn to_feature(&self) -> Feature {
        let point: Vec<f64> = vec![self.lon, self.lat];
        let point = Geometry::new(Value::Point(point));
        Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }

    pub fn to_json(&self) -> String {
        self.to_feature().to_string()
    }
}
