use std::{env, time::Instant};

use indicatif::ProgressIterator;
use osm_test::{CollisionDetection, Planet, PlanetGrid, Point};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/planet_grid_on_polygon.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    println!("generating grid");
    let mut planet_grid = PlanetGrid::new(500);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));

    println!("updating midpoints");
    planet_grid
        .spatial_partition
        .update_midpoint_with_planet(&planet);

    println!("checking if ray method works");
    planet_grid.spatial_partition.propagte_status(&planet);

    println!("generating points");
    let start = Instant::now();
    let n = 500_000;
    for _ in (0..n).progress() {
        let point = Point::random();
        if planet_grid.is_on_polygon(&point) {
            out_planet.points.push(point);
        }
    }
    println!("generating points took {:?} per point", start.elapsed() / n);

    out_planet.to_file(OUT_PLANET_PATH);
}
