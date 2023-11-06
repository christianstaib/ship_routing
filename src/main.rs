use std::{env, time::Instant};

use indicatif::{ProgressBar, ProgressIterator};
use osm_test::{CollisionDetection, Planet, PlanetGrid, Point};

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
    let mut planet_grid = PlanetGrid::new(500);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));
    println!("took {:?}", start.elapsed());

    // out_planet
    //     .arcs
    //     .extend(planet_grid.spatial_partition.get_leaf_polygons());

    println!("updating midpoints");
    planet_grid.spatial_partition.propagte_status();

    println!("generating points");
    let start = Instant::now();
    let n = 10_000;
    let pb = ProgressBar::new(n as u64);
    while out_planet.points.len() < n {
        let point = Point::random();
        if !planet_grid.is_on_polygon(&point) {
            pb.inc(1);
            out_planet.points.push(point);
        }
    }
    pb.finish();
    println!(
        "generating points took {:?} ({:?} per point)",
        start.elapsed(),
        start.elapsed() / n as u32
    );
    out_planet.to_geojson_file(OUT_PLANET_PATH);
}
