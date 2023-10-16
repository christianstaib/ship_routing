use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use geojson::{Feature, FeatureCollection, Geometry, Value};
use indicatif::ProgressIterator;
use osmpbf::{Element, ElementReader};

use crate::planet_elements::coordinate::GeodeticCoordinate;

pub struct RawPlanet {
    pub nodes: HashMap<i64, GeodeticCoordinate>,
    pub coastlines: Vec<Vec<i64>>,
}

pub struct Planet {
    pub coastlines: Vec<Vec<GeodeticCoordinate>>,
}

impl Planet {
    fn new(mut planet: RawPlanet) -> Self {
        let mut coastlines: Vec<Vec<GeodeticCoordinate>> = planet
            .coastlines
            .iter_mut()
            .progress()
            .map(|coastline| {
                coastline
                    .drain(0..)
                    .map(|node_id| planet.nodes[&node_id])
                    .collect()
            })
            .collect();

        coastlines.sort_unstable_by_key(|coastline| coastline.len());

        Self { coastlines }
    }

    pub fn from_path(path: &str) -> Self {
        let mut planet = RawPlanet::from_path(path);
        planet.close_coastline();
        Planet::new(planet)
    }

    pub fn to_file(&self, path: &str) {
        let mut feature_collection = FeatureCollection {
            bbox: None,
            features: Vec::new(),
            foreign_members: None,
        };

        for coastline in self.coastlines.iter() {
            let coastline = coastline
                .iter()
                .map(|node| vec![node.lon, node.lat])
                .collect();
            let geometry = Geometry::new(Value::Polygon(vec![coastline]));
            let feature = Feature {
                bbox: None,
                geometry: Some(geometry),
                id: None,
                properties: None,
                foreign_members: None,
            };
            feature_collection.features.push(feature);
        }

        let mut writer = BufWriter::new(File::create(path).unwrap());
        let feature_collection = feature_collection.to_string();

        writeln!(writer, "{}", feature_collection).unwrap();
        writer.flush().unwrap();
    }

    pub fn simplify(&mut self) {
        let coastlines: Vec<Vec<GeodeticCoordinate>> = self
            .coastlines
            .iter()
            .cloned()
            .filter(|coastline| coastline.len() >= 10_000)
            .map(|coastline| {
                if coastline.len() >= 10 {
                    return minimize_vec(coastline, 500);
                }
                coastline
            })
            .collect();
        self.coastlines = coastlines;
    }
}

pub fn minimize_vec<T: Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    let mut result = Vec::new();
    result.push(vec[0].clone()); // push the first element

    // iterate through the vector starting at nth element, stepping by n
    for i in (n..vec.len()).step_by(n) {
        result.push(vec[i].clone());
    }

    // push the last element if it hasn't been included in the previous step
    if vec.len() % n != 1 {
        result.push(vec.last().unwrap().clone());
    }

    result
}

impl RawPlanet {
    fn from_path(path: &str) -> RawPlanet {
        let mut nodes = HashMap::new();
        let mut coastlines = Vec::new();

        let reader = ElementReader::from_path(path).unwrap();
        reader
            .for_each(|element| match element {
                Element::DenseNode(dense_node) => {
                    nodes.insert(
                        dense_node.id(),
                        GeodeticCoordinate {
                            lat: dense_node.lat(),
                            lon: dense_node.lon(),
                        },
                    );
                }
                Element::Way(way) => {
                    if way
                        .tags()
                        .find(|(key, value)| *key == "natural" && (*value == "coastline"))
                        .is_some()
                    {
                        {
                            let mut sub_coastline = Vec::new();
                            way.refs().for_each(|node_id| {
                                sub_coastline.push(node_id);
                            });
                            coastlines.push(sub_coastline);
                        }
                    }
                }
                _ => (),
            })
            .unwrap();

        RawPlanet { nodes, coastlines }
    }

    fn close_coastline(&mut self) {
        let (mut open_coastlines, mut closed_coastlines): (Vec<_>, Vec<_>) = self
            .coastlines
            .drain(..)
            .partition(|coastline| coastline.first().unwrap() != coastline.last().unwrap());

        open_coastlines.sort_unstable_by_key(|coastline| coastline.first().cloned());

        while let Some(mut coastline) = open_coastlines.pop() {
            let mut last = coastline.last().unwrap().clone();
            while let Ok(to_append) = open_coastlines
                .binary_search_by_key(&last, |other_coastline| *other_coastline.first().unwrap())
            {
                let mut to_append = open_coastlines.remove(to_append);
                coastline.append(&mut to_append);
                last = coastline.last().unwrap().clone();
            }
            coastline.dedup();
            assert_eq!(coastline.first(), coastline.last());
            closed_coastlines.push(coastline);
        }
        assert!(open_coastlines.is_empty());
        self.coastlines = closed_coastlines;
    }
}
