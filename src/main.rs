use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{Planet, Point};

fn main() {
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    let start = Instant::now();
    let n = 100;
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
