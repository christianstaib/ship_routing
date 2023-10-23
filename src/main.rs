use std::fs::File;
use std::io::{BufWriter, Write};

use std::time::Instant;

use indicatif::ProgressIterator;

use crate::planet_elements::planet::Planet;
use crate::point_generator::PointGenerator;

pub mod planet_elements;
pub mod point_generator;

const PBF_PLANET: &str = "data/osm/planet-coastlinespbf-cleanedosmpbf.osm.pbf";
//const PGB_ANTARCTICA: &str = "data/osm/antarctica-latest.osm.pbf";
const _PLANET_PATH: &str = "data/geojson/planet.geojson";
const POINTS_PATH: &str = "data/geojson/points.geojson";
const N: usize = 100;

fn main() {
    _test_all();
}

fn _test_all() {
    let planet = Planet::from_osm(PBF_PLANET);
    println!("loaded pbf file");

    let start = Instant::now();
    let mut points = Planet::new();
    PointGenerator::new()
        .filter(|random_point| !planet.is_on_land_ray(random_point))
        .take(N)
        .progress_count(N as u64)
        .for_each(|random_point| points.points.push(random_point));
    let mut writer = BufWriter::new(File::create(POINTS_PATH).unwrap());
    points
        .points
        .iter()
        .for_each(|point| writeln!(writer, "{}", point.to_json()).unwrap());
    writer.flush().unwrap();

    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / N as u32
    );
}
