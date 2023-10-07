use std::io::Write;
use std::{fs::File, io::BufWriter};

use geojson::FeatureCollection;
use geojson_writer::coastline_to_feature;
use planet::Node;

use crate::planet::{does_intersect, Planet};

use geojson::{Feature, Geometry, Value};
mod exploration;
mod geojson_writer;

mod planet;

fn main() {
    // let path = "data/planet-coastlinespbf-cleanedosmpbf.osm.pbf";

    // let planet = Planet::from_path(path);

    // planet.to_file("test.geojson")

    let mut feature_collection = FeatureCollection {
        bbox: None,
        features: Vec::new(),
        foreign_members: None,
    };

    let p1 = Node { lat: 0.0, lon: 0.0 };
    let p2 = Node {
        lat: 90.0,
        lon: 0.0,
    };
    let p3 = Node {
        lat: 45.0,
        lon: -90.0,
    };
    let p4 = Node {
        lat: 45.0,
        lon: 90.0,
    };

    let coastline: Vec<Vec<f64>> = vec![p1, p2]
        .iter()
        .map(|node| vec![node.lon, node.lat])
        .collect();
    let geometry = Geometry::new(Value::LineString(coastline));
    let feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    };
    feature_collection.features.push(feature);
    let coastline: Vec<Vec<f64>> = vec![p3, p4]
        .iter()
        .map(|node| vec![node.lon, node.lat])
        .collect();
    let geometry = Geometry::new(Value::LineString(coastline));
    let feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    };
    feature_collection.features.push(feature);

    let result = does_intersect(&p1, &p2, &p3, &p4);
    println!("{}", result); // Output: true

    let p1 = Node { lat: 0.0, lon: 0.0 };
    let p2 = Node {
        lat: 10.0,
        lon: 0.0,
    };
    let p3 = Node {
        lat: 0.0,
        lon: 90.0,
    };
    let p4 = Node {
        lat: 90.0,
        lon: 90.0,
    };
    let coastline: Vec<Vec<f64>> = vec![p1, p2]
        .iter()
        .map(|node| vec![node.lon, node.lat])
        .collect();
    let geometry = Geometry::new(Value::LineString(coastline));
    let feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    };
    feature_collection.features.push(feature);
    let coastline: Vec<Vec<f64>> = vec![p3, p4]
        .iter()
        .map(|node| vec![node.lon, node.lat])
        .collect();
    let geometry = Geometry::new(Value::LineString(coastline));
    let feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    };
    feature_collection.features.push(feature);

    let result = does_intersect(&p1, &p2, &p3, &p4);
    println!("{}", result); // Output: false

    let mut writer = BufWriter::new(File::create("lines.geojson").unwrap());
    let feature_collection = feature_collection.to_string();
    write!(writer, "{}", feature_collection).unwrap();
    writer.flush().unwrap();
}
