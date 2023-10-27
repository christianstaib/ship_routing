use std::time::Instant;

use indicatif::ProgressIterator;
use rayon::prelude::*;

use crate::planet_elements::Arc;
use crate::planet_elements::Planet;
use crate::planet_elements::Point;
use crate::point_generator::PointGenerator;

pub mod planet_elements;
pub mod point_generator;

const EPSILON: f64 = 1e-10;

const PBF_PLANET: &str = "data/osm/planet-coastlinespbf-cleaned.osm.pbf";
const PLANET_PATH: &str = "data/geojson/planet.geojson";
const POINTS_PATH: &str = "data/geojson/points.geojson";
const N: usize = 250;

fn main() {
    _test_all();
}

fn _test_all() {
    let start = Instant::now();
    let mut planet = Planet::from_osm(PBF_PLANET);
    // let planet = Planet::from_json(PLANET_PATH).unwrap();
    println!("loaded pbf file in {:?}", start.elapsed());

    let start = Instant::now();

    let north_pole = Point::from_geodetic(90.0, 0.0);

    let mut points: Vec<Point> = PointGenerator::new().take(N).collect();
    let mut points_planet = Planet::new();

    // points
    //     .iter()
    //     .progress()
    //     .filter(|point| planet.is_on_land(point))
    //     .for_each(|point| points_planet.points.push(*point));

    let land_points: Vec<(f64, f64)> = points
        .iter()
        .progress()
        .map(|point| (point.lon, point.lat))
        .par_bridge()
        .filter(|(lon, lat)| planet.is_on_land(&Point::from_geodetic(*lat, *lon)))
        .collect();

    land_points
        .iter()
        .map(|(lon, lat)| Point::from_geodetic(*lat, *lon))
        .for_each(|random_point| {
            let line = Arc::new(random_point, north_pole);
            let intersections = planet.interctions(&line);
            planet.points.extend(intersections);
            // points_planet.points.push(random_point);

            let mut lat = random_point.lat;
            let lon = random_point.lon;
            let mut old_point = Point::from_geodetic(lat, lon);
            while lat <= 89.0 {
                lat += 0.1;
                let new_point = Point::from_geodetic(lat, lon);
                planet.lines.push(Arc::new(old_point, new_point));
                old_point = new_point;
            }
        });

    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / N as u32
    );

    planet.to_file(PLANET_PATH);
    points_planet.to_geojson_file(POINTS_PATH);
}
