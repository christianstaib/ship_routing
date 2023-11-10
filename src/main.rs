use indicatif::{ProgressBar, ProgressIterator};
use osm_test::{generate_network, radians_to_meter};
use osm_test::{
    meters_to_radians, Arc, CollisionDetection, Planet, PlanetGrid, Point, PointPlanetGrid, Polygon,
};
use rayon::prelude::{ParallelBridge, ParallelIterator};

use std::collections::HashMap;
use std::io::BufWriter;
use std::io::Write;
use std::{f64::consts::PI, fs::File};

const SEARCH_RADIUS: f64 = 35_000.0;

fn main() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const network_path: &str = "test.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();

    generate_network(&planet, network_path, OUT_PLANET_PATH);
}
