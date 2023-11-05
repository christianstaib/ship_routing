use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{Arc, CollisionDetector, Planet, Point, Polygon, Tiling};

fn main() {
    //test_clipping();
    osm_to_geojson();
}

fn osm_to_geojson() {
    const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/whole_planet.geojson";
    let planet = Planet::from_osm(PLANET_PATH);
    planet.to_file(OUT_PLANET_PATH);
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    // const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    // let planet = Planet::from_osm(PLANET_PATH);
    // planet.to_file(OUT_PLANET_PATH);
    let mut out_planet = Planet::new();
    planet
        .polygons
        .sort_by_key(|polygon| -1 * polygon.outline.len() as isize);
    planet.polygons = planet.polygons.into_iter().collect();
    out_planet.polygons = planet.polygons.clone();

    let start = Instant::now();
    let n = 10;
    for _ in (0..n).progress() {
        let point = Point::random();
        if planet.is_on_polygon(&point) {
            out_planet.points.push(point);
        }
    }
    println!("{:?}", start.elapsed() / n);

    out_planet.to_file(OUT_PLANET_PATH);
}

// fn make_good_line(line: Arc) -> Vec<Arc> {
//     let mut arcs = vec![line];
//     while arcs[0].central_angle() > 0.005 {
//         arcs = arcs
//             .iter()
//             .map(|arc| {
//                 let middle = arc.middle();
//                 vec![Arc::new(&arc.from(), &middle), Arc::new(&middle, &arc.to())]
//             })
//             .flatten()
//             .collect();
//     }
//     arcs.retain(|arc| (arc.from().lon() - arc.to().lon()).abs() < 10.0);
//     arcs
// }
