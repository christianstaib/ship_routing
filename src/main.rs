use std::{env, time::Instant};

use indicatif::ProgressIterator;
use osm_test::{CollisionDetection, Planet, PlanetGrid, Point};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    test_clipping();
}

fn test_clipping() {
    // const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    // let planet = Planet::from_file(PLANET_PATH).unwrap();
    const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    let planet = Planet::from_osm(PLANET_PATH);

    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/planet_grid_on_polygon.geojson";
    let mut out_planet = Planet::new();

    println!("generating grid");
    let mut planet_grid = PlanetGrid::new(500);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));

    // out_planet
    //     .arcs
    //     .extend(planet_grid.spatial_partition.get_leaf_polygons());

    println!("updating midpoints");
    planet_grid.spatial_partition.propagte_status();
    //    .update_midpoint_with_planet(&planet);

    // let out_planet = std::sync::Arc::new(Mutex::new(out_planet));
    // println!("checking if ray method works");
    // planet_grid
    //     .spatial_partition
    //     .propagte_status_test(&planet, out_planet.clone());
    // planet_grid.update_midpoints();
    //     out_planet.lock().unwrap().to_file(OUT_PLANET_PATH);

    println!("generating points");
    let start = Instant::now();
    let n = 250_000;
    for _ in (0..n).progress() {
        let point = Point::random();
        if !planet_grid.is_on_polygon(&point) {
            out_planet.points.push(point);
        }
    }
    println!("generating points took {:?} per point", start.elapsed() / n);
    out_planet.to_file(OUT_PLANET_PATH);
}
