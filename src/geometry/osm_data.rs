use std::collections::HashMap;

use osmpbf::{Element, ElementReader};

use super::{Planet, Point, Polygon};

/// a planet struct which ways are not cloesed
pub struct OsmData {
    pub nodes: HashMap<i64, Point>,
    pub coastlines: Vec<Vec<i64>>,
}

impl OsmData {
    pub fn to_planet(&self) -> Planet {
        let mut planet = Planet::new();
        planet.polygons.extend(
            self.coastlines
                .iter()
                .map(|coastline| {
                    coastline
                        .into_iter()
                        .map(|node_id| self.nodes[&node_id])
                        .collect()
                })
                .map(|coastline| Polygon::new(coastline)),
        );

        planet
    }

    pub fn from_path(path: &str) -> OsmData {
        let mut nodes = HashMap::new();
        let mut coastlines = Vec::new();

        let reader = ElementReader::from_path(path).unwrap();
        reader
            .for_each(|element| match element {
                Element::DenseNode(node) => {
                    nodes.insert(node.id(), Point::from_coordinate(node.lat(), node.lon()));
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

        let mut raw_osm_data = OsmData { nodes, coastlines };
        raw_osm_data.close_coastline();
        raw_osm_data
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
