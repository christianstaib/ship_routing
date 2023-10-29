use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Instant,
};

use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet, Point};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    // _test_inside_point();
    // _get_normal();
    _generate_points();
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

fn _test_inside_point() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/inside_point.geojson";

    // let planet = Planet::from_osm(PLANET_PATH);
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    planet
        .polygons
        .iter()
        .enumerate()
        // .skip(120)
        // .take(500)
        // .take(1)
        .progress()
        .for_each(|(i, polygon)| {
            // let mut stub = Planet::new();
            // let inside_point = polygon.get_inside_point(&mut stub);
            // if !planet.is_on_land(&inside_point) {
            //     polygon.get_inside_point(&mut out_planet);
            //     out_planet.points.push(inside_point);
            //     polygon
            //         .outline
            //         .windows(2)
            //         .for_each(|v| out_planet.lines.push(osm_test::Arc::new(v[0], v[1])));
            //     //out_planet.polygons.push(polygon.clone());
            //     println!("not inside {}", i);
            // }
        });
    out_planet.to_file(OUT_PLANET_PATH);
}

fn _generate_points() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const LAND_POINTS_PATH: &str = "tests/data/geojson/points_on_land.geojson";
    const WATER_POINTS_PATH: &str = "tests/data/geojson/points_on_water.geojson";
    const N: usize = 4_000;

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
        .for_each(|point| match planet.inside_is_on_land(&point) {
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
