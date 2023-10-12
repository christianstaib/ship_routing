use rand::Rng;

use std::io::Write;
use std::{fs::File, io::BufWriter};

use geojson::FeatureCollection;

use planet::GeodeticCoordinate;

use crate::planet::{does_intersect, Planet};

use geojson::{Feature, Geometry, Value};
mod exploration;
mod geojson_writer;

mod planet;

fn main() {
    let path = "data/planet-coastlinespbf-cleanedosmpbf.osm.pbf";

    let mut planet = Planet::from_path(path);

    planet.simplify();
    //let continent = planet.coastlines.last().unwrap().clone();

    planet.to_file("test.geojson");
    println!("wrote planet to geojson");

    let mut rng = rand::thread_rng();
    let mut feature_collection = FeatureCollection {
        bbox: None,
        features: Vec::new(),
        foreign_members: None,
    };

    for _ in 0..10_000 {
        let south_pole = GeodeticCoordinate { lat: 0.0, lon: 0.0 };
        let lat: f64 = rng.gen_range(-90.0..90.0);
        let lon: f64 = rng.gen_range(-180.0..180.0);
        let random_point = GeodeticCoordinate { lat, lon };

        let ray: Vec<f64> = vec![random_point.lon, random_point.lat];
        let ray = Geometry::new(Value::Point(ray));
        let ray = Feature {
            bbox: None,
            geometry: Some(ray),
            id: None,
            properties: None,
            foreign_members: None,
        };

        let intersections: Vec<bool> = planet
            .coastlines
            .iter()
            .map(|continent| {
                continent
                    .windows(2)
                    .map(|outline| {
                        does_intersect(&south_pole, &random_point, &outline[0], &outline[1])
                    })
                    .collect()
            })
            .map(|intersections: Vec<bool>| intersections.iter().filter(|&&x| x).count() % 2 == 1)
            .collect();

        if intersections.iter().all(|&x| x == false) {
            feature_collection.features.push(ray);
        }
    }

    let mut writer = BufWriter::new(File::create("points.geojson").unwrap());
    let feature_collection = feature_collection.to_string();
    write!(writer, "{}", feature_collection).unwrap();
    writer.flush().unwrap();
}
