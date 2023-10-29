use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Instant,
};

use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet, Point};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    _get_normal();
    // _generate_points();
}

fn _get_normal() {
    const NORMAL_PLANET_TEST: &str = "tests/data/test_geojson/normal_test.geojson";
    let mut planet = Planet::new();
    let a = Point::from_geodetic(0.0, 0.0);
    let b = Point::from_geodetic(-1.0, -1.0);
    let ab = osm_test::Arc::new(a, b);
    let middle = ab.middle();
    let destination = Point::destination_point(&middle, ab.initial_bearing() + (PI / 2.0), 0.1);
    let md = osm_test::Arc::new(middle, destination);

    println!("inital bearing is {}", ab.initial_bearing());

    planet.points.push(a);
    planet.points.push(b);
    planet.lines.push(ab);
    planet.lines.push(md);
    planet.points.push(middle);
    planet.to_file(NORMAL_PLANET_TEST);
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
