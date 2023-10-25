use std::time::Instant;

use indicatif::ProgressIterator;

use crate::planet_elements::planet::Planet;
use crate::point_generator::PointGenerator;

pub mod planet_elements;
pub mod point_generator;

const PBF_PLANET: &str = "data/osm/planet-coastlinespbf-cleaned.osm.pbf";
//const PGB_ANTARCTICA: &str = "data/osm/antarctica-latest.osm.pbf";
const _PLANET_PATH: &str = "data/geojson/planet.geojson";
const POINTS_PATH: &str = "data/geojson/points.geojson";
const FALSE_POSITIVE_PATH: &str = "data/geojson/fp_points.geojson";
const FALSE_NEGATIVE_PATH: &str = "data/geojson/fn_points.geojson";
const N: usize = 25_000;

fn main() {
    _test_all();
}

fn _test_all() {
    let start = Instant::now();
    // let planet = Planet::from_osm(PBF_PLANET);
    // planet.to_file(_PLANET_PATH);
    let planet = Planet::from_file(_PLANET_PATH).unwrap();
    println!("loaded pbf file in {:?}", start.elapsed());

    let start = Instant::now();
    let mut false_positive = Planet::new();
    let mut false_negative = Planet::new();
    PointGenerator::new()
        .take(N)
        .progress_count(N as u64)
        .enumerate()
        .for_each(|(i, random_point)| {
            let on_land = planet.is_on_land(&random_point);
            let true_on_land = planet.true_is_on_land(&random_point);
            if true_on_land == true && on_land == false {
                false_negative.points.push(random_point);
                println!(
                    "false positive: {}",
                    false_positive.points.len() as f32 / i as f32 * 100.0
                );
                println!(
                    "false negative: {}",
                    false_negative.points.len() as f32 / i as f32 * 100.0
                );
            } else if true_on_land == false && on_land == true {
                false_positive.points.push(random_point);
                println!(
                    "false positive: {}",
                    false_positive.points.len() as f32 / i as f32 * 100.0
                );
                println!(
                    "false negative: {}",
                    false_negative.points.len() as f32 / i as f32 * 100.0
                );
            }
        });

    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / N as u32
    );

    println!("false positive: {}", false_positive.points.len());
    println!("false negative: {}", false_negative.points.len());

    //points.polygons.extend(planet.polygons);
    false_positive.to_file(FALSE_POSITIVE_PATH);
    false_negative.to_file(FALSE_NEGATIVE_PATH);
}
