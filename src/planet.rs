use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use geojson::{Feature, FeatureCollection, Geometry, Value};
use indicatif::ProgressIterator;
use osmpbf::{Element, ElementReader};

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub lat: f64,
    pub lon: f64,
}

pub struct RawPlanet {
    pub nodes: HashMap<i64, Node>,
    pub coastlines: Vec<Vec<i64>>,
}

pub struct Planet {
    pub coastlines: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// https://blog.mbedded.ninja/mathematics/geometry/spherical-geometry/finding-the-intersection-of-two-arcs-that-lie-on-a-sphere/
pub fn does_intersect(p1: &Node, p2: &Node, p3: &Node, p4: &Node) -> bool {
    let p1 = Vec3::from_node(p1);
    let p2 = Vec3::from_node(p2);
    let p3 = Vec3::from_node(p3);
    let p4 = Vec3::from_node(p4);

    let n1 = p1.cross(&p2);
    let n2 = p3.cross(&p4);

    let l = n1.cross(&n2);

    let i1 = l.normalize();
    let mut i2 = i1.clone();
    i2.divide_by_scalar(-1.0);

    (Vec3::is_point_within_arc(&i1, &p1, &p2) && Vec3::is_point_within_arc(&i1, &p3, &p4))
        || (Vec3::is_point_within_arc(&i2, &p1, &p2) && Vec3::is_point_within_arc(&i2, &p3, &p4))
}

impl Vec3 {
    pub fn from_node(node: &Node) -> Self {
        let lat_rad = node.lat.to_radians();
        let lon_rad = node.lon.to_radians();

        Self {
            x: lat_rad.cos() * lon_rad.cos(),
            y: lat_rad.cos() * lon_rad.sin(),
            z: lat_rad.sin(),
        }
    }

    pub fn cross(self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn divide_by_scalar(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        if mag == 0.0 {
            panic!("Cannot normalize a zero vector");
        }
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_between(v1: &Vec3, v2: &Vec3) -> f64 {
        let angle = v1.dot(v2);
        let angle = angle / (v1.magnitude() * v2.magnitude());
        angle.acos()
    }

    pub fn is_point_within_arc(point: &Vec3, arc_start: &Vec3, arc_end: &Vec3) -> bool {
        let total_angle = Vec3::angle_between(arc_start, arc_end);
        let angle_sum = Vec3::angle_between(arc_start, point) + Vec3::angle_between(point, arc_end);
        (angle_sum - total_angle).abs() < 1e-6 // account for floating point inaccuracies
    }
}

impl Planet {
    fn new(mut planet: RawPlanet) -> Self {
        let coastlines: Vec<Vec<Node>> = planet
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
        Self { coastlines }
    }

    pub fn from_path(path: &str) -> Self {
        let mut planet = RawPlanet::from_path(path);
        planet.close_coastline();
        Planet::new(planet)
    }

    pub fn to_file(self, path: &str) {
        let mut feature_collection = FeatureCollection {
            bbox: None,
            features: Vec::new(),
            foreign_members: None,
        };

        let mut coastlines = self.coastlines.clone();

        coastlines = coastlines
            .into_iter()
            .filter(|coastline| coastline.len() >= 1_000)
            .map(|coastline| {
                if coastline.len() >= 1_000 {
                    return minimize_vec(coastline, 100);
                }
                coastline
            })
            .collect();

        for coastline in coastlines.iter() {
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
}

fn minimize_vec<T: Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    if vec.is_empty() || n == 0 {
        return Vec::new(); // return empty vector in case of an empty input vector or n being 0
    }

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
                    assert!((-90.0 <= dense_node.lat()) && (dense_node.lat() <= 90.0));
                    nodes.insert(
                        dense_node.id(),
                        Node {
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
