use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{Arc, CollisionDetector, Planet, Point, Polygon};

fn main() {
    test_clipping();
}

fn generate_rectangle(sw: &Point, ne: &Point) -> Polygon {
    assert!(sw.lat() < ne.lat());
    assert!(sw.lon() < ne.lon());

    let mut outline = vec![
        Point::from_geodetic(sw.lat(), sw.lon()),
        Point::from_geodetic(sw.lat(), ne.lon()),
        Point::from_geodetic(ne.lat(), ne.lon()),
        Point::from_geodetic(ne.lat(), sw.lon()),
        Point::from_geodetic(sw.lat(), sw.lon()),
    ];
    outline.dedup();
    assert!(outline.len() >= 4);
    Polygon::new(outline)
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();
    planet
        .polygons
        .sort_by_key(|polygon| -1 * polygon.outline.len() as isize);
    // planet.polygons = planet.polygons.iter().take(3).cloned().collect();
    // out_planet.polygons = planet.polygons.clone();

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
