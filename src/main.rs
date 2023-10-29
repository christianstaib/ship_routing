use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet, Point};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    _generate_points();
}

fn _generate_points() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const LAND_POINTS_PATH: &str = "tests/data/geojson/points_on_land.geojson";
    const WATER_POINTS_PATH: &str = "tests/data/geojson/points_on_water.geojson";
    const N: usize = 100_000;

    // let planet = Planet::from_osm(PLANET_PATH);
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let land_points = Arc::new(Mutex::new(Planet::new()));
    let water_points = Arc::new(Mutex::new(Planet::new()));

    let start = Instant::now();

    let points: Vec<Point> = PointGenerator::new().into_iter().take(N).collect();

    points
        .into_iter()
        .progress()
        .par_bridge()
        .for_each(|point| match planet.is_on_land(&point) {
            true => land_points.lock().unwrap().points.push(point),
            false => water_points.lock().unwrap().points.push(point),
        });

    println!(
        "took {:?} to generate {} points which is {:?} per point",
        start.elapsed(),
        N,
        start.elapsed() / N as u32
    );

    water_points.lock().unwrap().to_file(WATER_POINTS_PATH);
    land_points.lock().unwrap().to_file(LAND_POINTS_PATH);
}
