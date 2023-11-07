use std::{env, f64::consts::PI, sync::Mutex, time::Instant};

use indicatif::{ProgressBar, ProgressIterator};
use osm_test::{
    meters_to_radians, Arc, CollisionDetection, Planet, PlanetGrid, Point, PointPlanetGrid,
    Polygon, Tiling,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();
    // const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    // let planet = Planet::from_osm_file(PLANET_PATH);

    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/planet_grid_on_polygon.geojson";
    let mut out_planet = Planet::new();

    println!("generating grid");
    let start = Instant::now();
    let mut planet_grid = PlanetGrid::new(100);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));
    println!("took {:?}", start.elapsed());

    println!("updating midpoints");
    planet_grid.update_midpoints();

    println!("generating points");
    let mut point_grid = PointPlanetGrid::new(50);
    let start = Instant::now();
    let n = 4_000_000;
    let pb = ProgressBar::new(n as u64);
    let mut points = Vec::new();
    while points.len() < n {
        let point = Point::random();
        if !planet_grid.is_on_polygon(&point) {
            pb.inc(1);
            point_grid.add_point(&point);
            points.push(point);
        }
    }

    let out_planet = std::sync::Arc::new(Mutex::new(out_planet));
    points.iter().progress().par_bridge().for_each(|point| {
        for polygon in vec![ur(point), lr(point)] {
            let mut local_points = point_grid.get_points(&polygon);
            local_points.sort_by(|x, y| {
                Arc::new(point, x)
                    .central_angle()
                    .total_cmp(&Arc::new(point, y).central_angle())
            });
            let mut local_points = local_points.into_iter();
            if let Some(should_be_point) = local_points.next() {
                assert!((point.is_approximately_equal(&should_be_point)));
            }
            let mut local_points = local_points.into_iter();
            if let Some(target) = local_points.next() {
                out_planet
                    .lock()
                    .unwrap()
                    .arcs
                    .push(Arc::new(point, &target));
            }
        }
    });

    pb.finish();
    println!(
        "generating points took {:?} ({:?} per point)",
        start.elapsed(),
        start.elapsed() / n as u32
    );

    out_planet.lock().unwrap().to_geojson_file(OUT_PLANET_PATH);
}

fn ur(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 1.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 0.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        cloned_point,
    ])
}

fn lr(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 4.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 3.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        cloned_point,
    ])
}
fn ll(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 6.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 5.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 4.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        cloned_point,
    ])
}
fn ul(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 8.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 7.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        Point::destination_point(&point, 6.0 / 4.0 * PI, meters_to_radians(30_000.0)),
        cloned_point,
    ])
}
