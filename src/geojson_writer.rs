use std::{
    fs::File,
    io::{BufWriter, Write},
};

use geojson::{Feature, Geometry, Value};

use crate::planet_elements::coordinate::GeodeticCoordinate;

pub struct GeoJsonWriter {
    writer: BufWriter<File>,
    features: Vec<String>,
}

impl GeoJsonWriter {
    pub fn new(path: &str) -> Self {
        let writer = BufWriter::new(File::create(path).unwrap());
        //writeln!(writer, r#"{{"features":["#).unwrap();
        let features = Vec::new();
        Self { writer, features }
    }

    pub fn flush(mut self) {
        writeln!(self.writer, "{}", self.features.join("\n")).unwrap();
        //writeln!(self.writer, r#"],"type":"FeatureCollection"}}"#).unwrap();
        self.writer.flush().unwrap();
    }

    fn add_geometry(&mut self, geometry: Geometry) {
        let geometry = Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: None,
            foreign_members: None,
        };
        let geometry = geometry.to_string();
        self.features.push(geometry);
    }

    pub fn add_point(&mut self, point: &GeodeticCoordinate) {
        let point: Vec<f64> = vec![point.lon, point.lat];
        let point = Geometry::new(Value::Point(point));
        self.add_geometry(point);
    }

    pub fn add_polygon(&mut self, coastline: &Vec<GeodeticCoordinate>) {
        let polygon = coastline
            .iter()
            .map(|node| vec![node.lon, node.lat])
            .collect();
        let polygon = Geometry::new(Value::Polygon(vec![polygon]));
        self.add_geometry(polygon);
    }
}
