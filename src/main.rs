use std::{env, f64::consts::PI, sync::Mutex, time::Instant};

use indicatif::{ProgressBar, ProgressIterator};
use osm_test::{
    meters_to_radians, radians_to_meter, Arc, CollisionDetection, Planet, PlanetGrid, Point,
    PointPlanetGrid, Polygon, Tiling,
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

    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/water_network.geojson";
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

    out_planet.arcs.extend(
        points
            .iter()
            .progress()
            .par_bridge()
            .map(|point| {
                vec![ur(point), lr(point)]
                    .iter()
                    .filter_map(|polygon| {
                        let mut local_points = point_grid.get_points(&polygon);
                        local_points.sort_by_cached_key(|x| {
                            (radians_to_meter(Arc::new(point, x).central_angle()) / 100_0.0) as u64
                            // sort by mm (100 cm in a m, 10 mm in a cm). I dont want to sory by
                            // comparing f64 as this would require to recomputer central_angle()
                            // every time.
                        });

                        // .get(1) is point
                        if let Some(target) = local_points.get(2) {
                            return Some(Arc::new(point, &target));
                        }
                        None
                    })
                    .collect::<Vec<Arc>>()
            })
            .flatten()
            .collect::<Vec<Arc>>(),
    );

    pb.finish();
    println!(
        "generating points took {:?} ({:?} per point)",
        start.elapsed(),
        start.elapsed() / n as u32
    );

    out_planet.to_geojson_file(OUT_PLANET_PATH);
}

fn ur(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(3_000.0)),
        Point::destination_point(&point, 1.0 / 4.0 * PI, meters_to_radians(3_000.0)),
        Point::destination_point(&point, 0.0 / 4.0 * PI, meters_to_radians(3_000.0)),
        cloned_point,
    ])
}

fn lr(point: &Point) -> Polygon {
    let cloned_point = point.clone();
    Polygon::new(vec![
        cloned_point,
        Point::destination_point(&point, 4.0 / 4.0 * PI, meters_to_radians(3_000.0)),
        Point::destination_point(&point, 3.0 / 4.0 * PI, meters_to_radians(3_000.0)),
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(3_000.0)),
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
