use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet};

fn main() {
    const PLANET_PATH: &str = "data/test_geojson/planet.geojson";
    // const PLANET_PATH: &str = "data/osm/planet-coastlines.osm.pbf";
    const LAND_POINTS_PATH: &str = "data/geojson/points_on_land.geojson";
    const WATER_POINTS_PATH: &str = "data/geojson/points_on_water.geojson";

    // let planet = Planet::from_osm(PLANET_PATH);
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut land_points = Planet::new();
    let mut water_points = Planet::new();

    let start = Instant::now();
    PointGenerator::new()
        .into_iter()
        .take(10_000)
        .progress_count(10_000)
        .for_each(|point| match planet.fast_is_on_land(&point) {
            true => land_points.points.push(point),
            false => water_points.points.push(point),
        });

    water_points.to_file(WATER_POINTS_PATH);
    println!(
        "took {:?} to generate {} points which is {:?} per point",
        start.elapsed(),
        100_000,
        start.elapsed() / 100_000
    );

    land_points.to_file(LAND_POINTS_PATH);
}
