use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use geojson::{Feature, FeatureCollection, Value};

use super::{coordinate::Coordinate, line::Line, polygon::Polygon, raw_osm_data::RawOsmData};

pub struct Planet {
    pub polygons: Vec<Polygon>,
    pub points: Vec<Coordinate>,
    pub lines: Vec<Line>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            polygons: Vec::new(),
            points: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Planet, Box<dyn Error>> {
        let mut planet = Planet::new();
        for line in json.lines() {
            if let Some(geomtry) = Feature::from_str(line)?.geometry {
                match geomtry.value {
                    Value::Point(point) => planet
                        .points
                        .push(Coordinate::from_geodetic(point[1], point[0])),
                    Value::Polygon(polygon) => {
                        let points = polygon[0]
                            .iter()
                            .map(|x| Coordinate::from_geodetic(x[1], x[0]))
                            .collect();
                        planet.polygons.push(Polygon::new(points))
                    }
                    _ => (),
                }
            }
        }
        Ok(planet)
    }

    pub fn from_file(path: &str) -> Result<Planet, Box<dyn Error>> {
        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        Planet::from_json(json.as_str())
    }

    pub fn from_osm(path: &str) -> Self {
        let raw_osm_data = RawOsmData::from_path(path);
        raw_osm_data.to_planet()
    }

    pub fn interctions(&self, line: &Line) -> Vec<Coordinate> {
        self.polygons
            .iter()
            .map(|polygon| polygon.intersections(line))
            .flatten()
            .collect()
    }

    pub fn is_on_land(&self, point: &Coordinate) -> bool {
        let north_pole = Coordinate::from_geodetic(90.0, 0.0);
        self.polygons
            .iter()
            .any(|polygon| polygon.contains(point, &north_pole))
    }

    pub fn to_geojson(&self) -> String {
        let mut features = Vec::new();
        features.extend(self.points.iter().map(|point| point.to_feature()));
        features.extend(self.polygons.iter().map(|polygon| polygon.to_feature()));
        features.extend(self.lines.iter().map(|polygon| polygon.to_feature()));
        FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        }
        .to_string()
    }

    pub fn to_geojson_file(&self, path: &str) {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        write!(writer, "{}", self.to_geojson()).unwrap();
        writer.flush().unwrap();
    }

    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.extend(self.polygons.iter().map(|polygon| polygon.to_json() + "\n"));
        json.extend(self.points.iter().map(|polygon| polygon.to_json() + "\n"));
        json.extend(self.lines.iter().map(|polygon| polygon.to_json() + "\n"));
        json
    }

    pub fn to_file(&self, path: &str) {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        write!(writer, "{}", self.to_json()).unwrap();
        writer.flush().unwrap();
    }
}
