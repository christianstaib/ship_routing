use geojson::{Feature, Geometry, Value};
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeodeticCoordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SphericalCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeodeticCoordinate {
    pub fn to_vector3(&self) -> Vector3<f64> {
        let lat_rad = self.lat.to_radians();
        let lon_rad = self.lon.to_radians();

        Vector3::new(
            lat_rad.cos() * lon_rad.cos(),
            lat_rad.cos() * lon_rad.sin(),
            lat_rad.sin(),
        )
    }

    pub fn to_json(&self) -> String {
        let point: Vec<f64> = vec![self.lon, self.lat];
        let point = Geometry::new(Value::Point(point));
        let geometry = Feature {
            bbox: None,
            geometry: Some(point),
            id: None,
            properties: None,
            foreign_members: None,
        };
        geometry.to_string()
    }
}

impl SphericalCoordinate {
    pub fn from_node(node: &GeodeticCoordinate) -> Self {
        let lat_rad = node.lat.to_radians();
        let lon_rad = node.lon.to_radians();

        Self {
            x: lat_rad.cos() * lon_rad.cos(),
            y: lat_rad.cos() * lon_rad.sin(),
            z: lat_rad.sin(),
        }
    }
}
