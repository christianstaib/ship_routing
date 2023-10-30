use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Instant,
};

use geo::polygon;
use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet, Point, Polygon};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    // _test_inside_point();
    // _get_normal();
    // _generate_points();
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/test_clipping.geojson";

    // let planet = Planet::from_osm(PLANET_PATH);
    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    let a = Point::from_geodetic(0.0, 0.0);
    let b = Point::from_geodetic(0.0, 3.0);
    let c = Point::from_geodetic(1.0, 3.0);
    let d = Point::from_geodetic(1.0, 1.0);
    let e = Point::from_geodetic(2.0, 1.0);
    let f = Point::from_geodetic(2.0, 3.0);
    let g = Point::from_geodetic(3.0, 3.0);
    let h = Point::from_geodetic(3.0, 0.0);

    let outline = vec![a, b, c, d, e, f, g, h, a];
    let polygon = Polygon::new(outline);
    // out_planet.polygons.push(polygon.clone());

    let a = Point::from_geodetic(-1.0, 2.0);
    let b = Point::from_geodetic(-1.0, 4.0);
    let c = Point::from_geodetic(4.0, 4.0);
    let d = Point::from_geodetic(4.0, 2.0);

    let outline = vec![a, b, c, d, a];
    let clipping_polygon = Polygon::new(outline);
    // out_planet.polygons.push(clipping_polygon.clone());

    let clipped = polygon.clip(&clipping_polygon, &mut out_planet);

    out_planet.polygons.extend(clipped);
    //clipping_polygon.clip(&polygon, &mut out_planet);

    // let intersections = polygon.intersections_polygon(&clipping_polygon);
    // intersections
    //     .iter()
    //     .for_each(|(self_arc, other_arc, intersection)| match intersection {
    //         osm_test::PointClassification::InIntersection(point) => {
    //             out_planet.points.push(point.clone())
    //         }
    //         _ => (),
    //     });

    out_planet.to_file(OUT_PLANET_PATH);
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
    const N: usize = 4_000_000;

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
