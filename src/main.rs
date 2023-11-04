use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{CollisionDetector, Planet, Point, Tiling};

fn main() {
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    // const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    // let planet = Planet::from_osm(PLANET_PATH);
    // planet.to_file(OUT_PLANET_PATH);
    let mut out_planet = Planet::new();

    let base_pixels = Tiling::base_tiling();

    let mut quadtree = CollisionDetector::new(&base_pixels);

    println!("adding polygons to partition");
    planet
        .polygons
        .iter()
        .cloned()
        .progress()
        .for_each(|polygon| quadtree.add_polygon(polygon));

    println!("updating midpoints");
    quadtree.update_midpoints();

    println!("generating points");

    let start = Instant::now();
    let n = 10_000;
    for _ in (0..n).progress() {
        let point = Point::random();
        if quadtree.is_on_polygon(&point) {
            out_planet.points.push(point);
        }
    }
    println!("{:?}", start.elapsed() / n);

    out_planet.to_file(OUT_PLANET_PATH);
}
