use std::{
    fs::File,
    io::{BufWriter, Write},
};

use geojson::{Feature, FeatureCollection, Geometry, Value};

use crate::planet_elements::coordinate::GeodeticCoordinate;

pub struct PlanetWriter {
    feature_collection: FeatureCollection,
}

impl PlanetWriter {
    pub fn new() -> Self {
        let feature_collection = FeatureCollection {
            bbox: None,
            features: Vec::new(),
            foreign_members: None,
        };
        Self { feature_collection }
    }

    pub fn write(&self, path: &str) {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        write!(writer, "{}", self.feature_collection.to_string()).unwrap();
        writer.flush().unwrap();
    }

    pub fn add_point(&mut self, point: &GeodeticCoordinate) {
        let ray: Vec<f64> = vec![point.lon, point.lat];
        let ray = Geometry::new(Value::Point(ray));
        let ray = Feature {
            bbox: None,
            geometry: Some(ray),
            id: None,
            properties: None,
            foreign_members: None,
        };
        self.feature_collection.features.push(ray);
    }
}
