use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use geojson::{FeatureCollection, Value};

use crate::{Arc, CollisionDetection, Point, Polygon, RawOsmData, SolidShape};

#[derive(Clone)]
pub struct Planet {
    pub polygons: Vec<Polygon>,
    pub points: Vec<Point>,
    pub arcs: Vec<Arc>,
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

    fn intersects_polygon(&self, arc: &Arc) -> bool {
        self.polygons
            .iter()
            .filter(|polygon| polygon.intersects(arc))
            .next()
            .is_some()
    }
}

impl Planet {
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            points: Vec::new(),
            arcs: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Planet, Box<dyn Error>> {
        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        Planet::from_geojson(json.as_str())
    }

    pub fn from_osm(path: &str) -> Self {
        let raw_osm_data = RawOsmData::from_path(path);
        raw_osm_data.to_planet()
    }

    pub fn from_geojson(json: &str) -> Result<Planet, Box<dyn Error>> {
        let feature_collection = FeatureCollection::from_str(json)?;
        let mut planet = Planet::new();
        feature_collection
            .into_iter()
            .filter_map(|feature| feature.geometry)
            .for_each(|geometry| match geometry.value {
                Value::Point(point) => planet.points.push(Point::from_vec(point).unwrap()),
                Value::Polygon(polygon) => planet
                    .polygons
                    .push(Polygon::from_vec(polygon[0].clone()).unwrap()),
                Value::LineString(line) => planet.arcs.push(Arc::from_vec(line).unwrap()),
                _ => (),
            });

        Ok(planet)
    }

    pub fn intersections(&self, arc: &Arc) -> Vec<Point> {
        self.polygons
            .iter()
            .map(|polygon| polygon.intersections(arc))
            .flatten()
            .collect()
    }

    pub fn to_geojson(&self) -> String {
        let mut features = Vec::new();
        features.extend(self.points.iter().map(|point| point.to_feature()));
        features.extend(self.polygons.iter().map(|polygon| polygon.to_feature()));
        features.extend(self.arcs.iter().map(|line| line.to_feature()));
        FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        }
        .to_string()
    }

    pub fn to_file(&self, path: &str) {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        write!(writer, "{}", self.to_geojson()).unwrap();
        writer.flush().unwrap();
    }
}