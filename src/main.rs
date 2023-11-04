use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{Arc, CollisionDetector, Planet, Point, Polygon};

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

    let np = Point::from_geodetic(90.0, 0.0);
    let sp = Point::from_geodetic(-90.0, 0.0);
    let mid_ring: Vec<f64> = vec![180.0, -90.0, 0.0, 90.0, 180.0];
    let mid_ring: Vec<Point> = mid_ring
        .iter()
        .map(|&lon| Point::from_geodetic(0.0, lon))
        .collect();
    let upper_ring: Vec<Point> = mid_ring
        .iter()
        .map(|mid| Arc::new(&mid, &np).middle())
        .collect();
    let lower_ring: Vec<Point> = mid_ring
        .iter()
        .map(|mid| Arc::new(&mid, &sp).middle())
        .collect();
    let mid_ring: Vec<f64> = vec![-135.0, -45.0, 45.0, 135.0, -135.0];
    let mid_ring: Vec<Point> = mid_ring
        .iter()
        .map(|&lon| Point::from_geodetic(0.0, lon))
        .collect();

    let mut base_pixels = Vec::new();

    for i in 0..4 {
        let polygon = Polygon::new(vec![
            upper_ring[i].clone(),
            mid_ring[i].clone(),
            upper_ring[i + 1].clone(),
            np.clone(),
            upper_ring[i].clone(),
        ]);
        base_pixels.push(polygon);

        let polygon = Polygon::new(vec![
            lower_ring[i].clone(),
            sp.clone(),
            lower_ring[i + 1].clone(),
            mid_ring[i].clone(),
            lower_ring[i].clone(),
        ]);
        base_pixels.push(polygon);

        let polygon = Polygon::new(vec![
            mid_ring[i].clone(),
            lower_ring[i + 1].clone(),
            mid_ring[i + 1].clone(),
            upper_ring[i + 1].clone(),
            mid_ring[i].clone(),
        ]);
        base_pixels.push(polygon);
    }

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
