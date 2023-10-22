use std::f64::consts::PI;
use std::time::Instant;

use indicatif::ProgressIterator;
use planet_elements::coordinate::{subtended_angle, GeodeticCoordinate};
use planet_elements::polygon::Polygon;

use crate::geojson_writer::GeoJsonWriter;
use crate::planet::Planet;
use crate::planet_elements::new_planet::NewPlanet;
use crate::point_generator::PointGenerator;

pub mod geojson_writer;
mod planet;
pub mod planet_elements;
pub mod point_generator;

fn main() {
    _test_all();
}

fn _test_inside() {
    let a = GeodeticCoordinate {
        lat: 90.0,
        lon: 0.0,
    };
    let b = GeodeticCoordinate { lat: 0.0, lon: 0.0 };
    let c = GeodeticCoordinate {
        lat: 0.0,
        lon: 90.0,
    };

    let polygon = Polygon {
        outline: vec![a, b, c, a],
    };

    let points_path = "data/geojson/points.geojson";
    let n = 1_000_000;
    let mut geojson_writer = GeoJsonWriter::new(points_path);
    PointGenerator::new()
        .filter(|random_point| !polygon.contains_winding(random_point))
        .take(n)
        .progress_count(n as u64)
        .for_each(|random_point| geojson_writer.add_point(&random_point));
    geojson_writer.flush();
}

fn _test_subtended_angle() {
    let a = GeodeticCoordinate {
        lat: 90.0,
        lon: 0.0,
    };
    let b = GeodeticCoordinate { lat: 0.0, lon: 0.0 };
    let c = GeodeticCoordinate {
        lat: 0.0,
        lon: 90.0,
    };

    let angle_rad = subtended_angle(&a, &b, &c);
    let angle = angle_rad * 180.0 / PI;
    assert_eq!(angle, 90.0);
}

fn _test_all() {
    let pbf_path = "data/osm/planet-coastlinespbf-cleanedosmpbf.osm.pbf";
    //let pbf_path = "data/osm/antarctica-latest.osm.pbf";
    let planet_path = "data/geojson/planet.geojson";
    let points_path = "data/geojson/points.geojson";
    let n = 50_000;

    let start = Instant::now();
    let planet = Planet::from_path(pbf_path);
    println!("loading pbf file took {:?}", start.elapsed());

    let start = Instant::now();
    planet.to_file(planet_path);
    println!("writing file took {:?}", start.elapsed());

    let planet = NewPlanet::from_planet(&planet);

    //planet.land_mass.iter().progress().for_each(|polygon| {
    //    let point = polygon.outline.first().unwrap().clone();
    //    planet
    //        .land_mass
    //        .iter()
    //        .filter(|&other| other == polygon)
    //        .filter(|other| other.contains(&point))
    //        .for_each(|other| println!("is inside another"));
    //});

    let start = Instant::now();
    let mut geojson_writer = GeoJsonWriter::new(points_path);
    PointGenerator::new()
        .filter(|random_point| !planet.is_on_land_ray(random_point))
        .take(n)
        .progress_count(n as u64)
        .for_each(|random_point| geojson_writer.add_point(&random_point));
    geojson_writer.flush();
    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / n as u32
    );
}
