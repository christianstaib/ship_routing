use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use geojson::{FeatureCollection, Value};

use crate::{geometry::Arc, geometry::Linestring, geometry::Point, geometry::Polygon};

use super::{collision_detection::CollisionDetection, Contains, OsmData};

#[derive(Clone)]
pub struct Planet {
    pub polygons: Vec<Polygon>,
    pub points: Vec<Point>,
    pub arcs: Vec<Arc>,
    pub linestrings: Vec<Linestring>,
}

impl CollisionDetection for Planet {
    fn add_polygon(&mut self, polygon: &Polygon) {
        self.polygons.push(polygon.clone());
    }

    fn is_on_polygon(&self, point: &Point) -> bool {
        self.polygons
            .iter()
            .filter(|polygon| polygon.contains(point))
            .next()
            .is_some()
    }

    fn intersects_polygon(&self, _arc: &Arc) -> bool {
        todo!()
    }
}

impl Planet {
    /// Creates a new, empty planet.
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            points: Vec::new(),
            arcs: Vec::new(),
            linestrings: Vec::new(),
        }
    }

    /// Returns the intersection points between all polygons and the arc.
    pub fn intersections(&self, arc: &Arc) -> Vec<Point> {
        self.polygons
            .iter()
            .map(|polygon| polygon.intersections(arc))
            .flatten()
            .collect()
    }

    pub fn from_osm_file(path: &str) -> Self {
        let raw_osm_data = OsmData::from_path(path);
        raw_osm_data.to_planet()
    }

    pub fn from_geojson_str(json: &str) -> Result<Planet, Box<dyn Error>> {
        let feature_collection = FeatureCollection::from_str(json)?;
        let mut planet = Planet::new();
        feature_collection
            .into_iter()
            .filter_map(|feature| feature.geometry)
            .for_each(|geometry| match geometry.value {
                Value::Point(point) => planet.points.push(Point::from_geojson_vec(point)),
                Value::Polygon(polygon) => planet
                    .polygons
                    .push(Polygon::from_geojson_vec(polygon[0].clone())),
                Value::LineString(line) => planet.arcs.push(Arc::from_geojson_vec(line)),
                _ => (),
            });

        Ok(planet)
    }

    pub fn from_geojson_file(path: &str) -> Result<Planet, Box<dyn Error>> {
        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        Planet::from_geojson_str(json.as_str())
    }

    pub fn to_geojson_str(&self) -> String {
        let mut features = Vec::new();
        features.extend(self.points.iter().map(|point| point.to_feature()));
        features.extend(self.polygons.iter().map(|polygon| polygon.to_feature()));
        features.extend(self.arcs.iter().map(|line| line.to_feature()));
        features.extend(
            self.linestrings
                .iter()
                .map(|linestring| linestring.to_feature()),
        );

        let mut writer = String::new();
        writer += r#"{"type":"FeatureCollection","features":["#;

        let mut features = features.into_iter().peekable();
        while let Some(feature) = features.next() {
            if features.peek().is_some() {
                writer += &format!("{},", feature.to_string());
            } else {
                writer += &format!("{}", feature.to_string());
            }
        }
        writer += r#"]}"#;
        writer
    }

    pub fn to_geojson_file(&self, path: &str) {
        let mut features = Vec::new();
        features.extend(self.points.iter().map(|point| point.to_feature()));
        features.extend(self.polygons.iter().map(|polygon| polygon.to_feature()));
        features.extend(self.arcs.iter().map(|line| line.to_feature()));
        features.extend(
            self.linestrings
                .iter()
                .map(|linestring| linestring.to_feature()),
        );

        let mut writer = BufWriter::new(File::create(path).unwrap());
        writeln!(writer, r#"{{"type":"FeatureCollection","features":["#,).unwrap();

        let mut features = features.into_iter().peekable();
        while let Some(feature) = features.next() {
            if features.peek().is_some() {
                writeln!(writer, "{},", feature.to_string()).unwrap();
            } else {
                writeln!(writer, "{}", feature.to_string()).unwrap();
            }
        }
        writeln!(writer, r#"]}}"#,).unwrap();
        writer.flush().unwrap();
    }
}
