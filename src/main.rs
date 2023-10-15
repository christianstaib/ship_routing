use std::time::Instant;

use indicatif::ProgressIterator;
use rayon::prelude::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};

use crate::planet::Planet;
use crate::planet_elements::coordinate::GeodeticCoordinate;
use crate::planet_elements::new_planet::NewPlanet;
use crate::planet_elements::polygon::Polygon;
use crate::planet_writer::PlanetWriter;
use crate::point_generator::PointGenerator;

mod exploration;

mod planet;
pub mod planet_elements;
pub mod planet_writer;
pub mod point_generator;

fn main() {
    let path = "data/planet-coastlinespbf-cleanedosmpbf.osm.pbf";

    let mut planet = Planet::from_path(path);

    planet.to_file("geodata/planet_detailed.geo.json");
    planet.simplify();
    planet.to_file("geodata/planet_simplified.geo.json");

    println!("wrote planet to geojson");

    planet
        .coastlines
        .sort_unstable_by_key(|outline| -1 * outline.len() as isize);

    let land_mass: Vec<Polygon> = planet
        .coastlines
        .iter()
        .cloned()
        .map(|outline| Polygon::new(outline))
        .collect();

    let planet = NewPlanet { land_mass };
    let mut planet_writer = PlanetWriter::new();

    let n = 10;
    let start = Instant::now();
    let point_point = PointGenerator::new();
    let points: Vec<GeodeticCoordinate> = point_point.take(n).progress_count(n as u64).collect();

    let points: Vec<GeodeticCoordinate> = points
        .into_par_iter()
        .filter(|random_point| !planet.is_on_land(random_point))
        .collect();

    points
        .iter()
        .for_each(|random_point| planet_writer.add_point(&random_point));
    println!("took {:?}", start.elapsed());

    planet_writer.write(format!("geodata/points_{}.geo.json", n).as_str());
}
