use std::time::Instant;

use indicatif::ProgressIterator;

use crate::planet_elements::planet::Planet;
use crate::point_generator::PointGenerator;

pub mod planet_elements;
pub mod point_generator;

const PBF_PLANET: &str = "data/osm/planet-coastlinespbf-cleaned.osm.pbf";
//const PGB_ANTARCTICA: &str = "data/osm/antarctica-latest.osm.pbf";
const _PLANET_PATH: &str = "data/osm/planet.geojson";
const POINTS_PATH: &str = "data/geojson/points.geojson";
const N: usize = 5_000;

fn main() {
    _test_all();
}

fn _test_all() {
    let start = Instant::now();
    let planet = Planet::from_osm(PBF_PLANET);
    planet.to_file(_PLANET_PATH);
    //let planet = Planet::from_file(_PLANET_PATH).unwrap();
    //planet.to_file(_PLANET_PATH);
    println!("loaded pbf file in {:?}", start.elapsed());

    let start = Instant::now();
    let mut points = Planet::new();
    PointGenerator::new()
        .filter(|random_point| !planet.is_on_land_ray(random_point))
        .take(N)
        .progress_count(N as u64)
        .for_each(|random_point| points.points.push(random_point));

    points.to_file(POINTS_PATH);
    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / N as u32
    );
}
