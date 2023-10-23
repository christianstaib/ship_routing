use std::{collections::HashMap, time::Instant};

use indicatif::ProgressIterator;
use osmpbf::{Element, ElementReader};

use crate::{geojson_writer::GeoJsonWriter, planet_elements::coordinate::GeodeticCoordinate};

/// a planet struct which ways are not cloesed
pub struct RawPlanet {
    pub nodes: HashMap<i64, GeodeticCoordinate>,
    pub coastlines: Vec<Vec<i64>>,
}

pub struct Planet {
    pub coastlines: Vec<Vec<GeodeticCoordinate>>,
}

impl Planet {
    fn new(mut planet: RawPlanet) -> Self {
        let coastlines: Vec<Vec<GeodeticCoordinate>> = planet
            .coastlines
            .iter_mut()
            .progress()
            .map(|coastline| {
                coastline
                    .into_iter()
                    .map(|node_id| planet.nodes[&node_id])
                    .collect()
            })
            .collect();

        Self { coastlines }
    }

    pub fn from_path(path: &str) -> Self {
        let mut planet = RawPlanet::from_path(path);
        planet.close_coastline();
        Self::new(planet)
    }

    pub fn to_file(&self, path: &str) {
        let mut writer = GeoJsonWriter::new(path);

        self.coastlines
            .iter()
            .for_each(|coastline| writer.add_polygon(coastline));

        writer.flush();
    }
}

impl RawPlanet {
    fn from_path(path: &str) -> RawPlanet {
        let mut nodes = HashMap::new();
        let mut coastlines = Vec::new();

        let reader = ElementReader::from_path(path).unwrap();
        reader
            .for_each(|element| match element {
                Element::DenseNode(node) => {
                    nodes.insert(
                        node.id(),
                        GeodeticCoordinate {
                            lat: node.lat(),
                            lon: node.lon(),
                        },
                    );
                }
                Element::Way(way) => {
                    if way
                        .tags()
                        .find(|(key, value)| *key == "natural" && (*value == "coastline"))
                        .is_some()
                    {
                        coastlines.push(way.refs().collect());
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
