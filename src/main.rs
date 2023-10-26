use std::time::Instant;

use indicatif::ProgressIterator;

use crate::planet_elements::Line;
use crate::planet_elements::Planet;
use crate::planet_elements::Point;
use crate::point_generator::PointGenerator;

pub mod planet_elements;
pub mod point_generator;

const EPSILON: f64 = 1e-10;

const PBF_PLANET: &str = "data/osm/planet-coastlinespbf-cleaned.osm.pbf";
const PLANET_PATH: &str = "data/geojson/planet.geojson";
const POINTS_PATH: &str = "data/geojson/points.geojson";
const N: usize = 1;

fn main() {
    _test_all();
}

fn _test_all() {
    let start = Instant::now();
    let planet = Planet::from_osm(PBF_PLANET);
    // let planet = Planet::from_json(PLANET_PATH).unwrap();
    println!("loaded pbf file in {:?}", start.elapsed());

    let start = Instant::now();

    let north_pole = Point::from_geodetic(90.0, 0.0);

    let mut points: Vec<Point> = PointGenerator::new().take(N).collect();
    let mut points_planet = Planet::new();

    points
        .iter_mut()
        .progress()
        //.filter(|random_point| planet.is_on_land(&random_point))
        .for_each(|&mut random_point| {
            let line = Line::new(random_point, north_pole);
            let intersections = planet.interctions(&line);
            let min_intersection_lat = intersections
                .iter()
                .map(|cord| cord.lat)
                .reduce(f64::min)
                .unwrap_or(90.0);
            //if min_intersection_lat < random_point.lat {
            //    panic!(
            //        "wrong intersections, min:{}, random:{}",
            //        min_intersection_lat, random_point.lat
            //    );
            //}
            points_planet.points.extend(intersections);
            points_planet.points.push(random_point);

            let mut lat = random_point.lat;
            let lon = random_point.lon;
            let mut old_point = Point::from_geodetic(lat, lon);
            while lat <= 89.0 {
                lat += 0.1;
                let new_point = Point::from_geodetic(lat, lon);
                points_planet.lines.push(Line::new(old_point, new_point));
                old_point = new_point;
            }
        });

    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / N as u32
    );

    // planet.to_file(PLANET_PATH);
    points_planet.to_geojson_file(POINTS_PATH);
}
