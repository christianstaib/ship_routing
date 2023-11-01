use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use geojson::{FeatureCollection, Value};

use super::{Arc, Point, Polygon, RawOsmData};

#[derive(Clone)]
pub struct Planet {
    pub polygons: Vec<Polygon>,
    pub points: Vec<Point>,
    pub lines: Vec<Arc>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            points: Vec::new(),
            lines: Vec::new(),
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

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        self.polygons
            .iter()
            .map(|polygon| polygon.intersections(line))
            .flatten()
            .collect()
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        self.polygons
            .iter()
            .any(|polygon| polygon.contains_inside(point))
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
                Value::LineString(line) => planet.lines.push(Arc::from_vec(line).unwrap()),
                _ => (),
            });

        Ok(planet)
    }

    pub fn to_geojson(&self) -> String {
        let mut features = Vec::new();
        features.extend(self.points.iter().map(|point| point.to_feature()));
        features.extend(self.polygons.iter().map(|polygon| polygon.to_feature()));
        features.extend(self.lines.iter().map(|line| line.to_feature()));
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
