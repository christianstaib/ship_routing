use std::time::Instant;

use indicatif::ProgressIterator;

use crate::planet::Planet;
use crate::planet_elements::new_planet::NewPlanet;
use crate::planet_writer::PlanetWriter;
use crate::point_generator::PointGenerator;

mod exploration;

mod planet;
pub mod planet_elements;
pub mod planet_writer;
pub mod point_generator;

fn main() {
    let pbf_path = "data/planet-coastlinespbf-cleanedosmpbf.osm.pbf";
    //let detailed_geojson_path = "geodata/planet_detailed.geo.json";
    let simplified_geojson_path = "geodata/planet_simplified.geo.json";
    let points_path = "geodata/points.geo.json";
    let n = 10;

    let start = Instant::now();
    let mut planet = Planet::from_path(pbf_path);
    println!("loading pbf file took {:?}", start.elapsed());

    planet.simplify();
    //planet.to_file(detailed_geojson_path);

    let start = Instant::now();
    planet.to_file(simplified_geojson_path);
    println!("writing file took {:?}", start.elapsed());

    let planet = NewPlanet::from_planet(&planet);

    let start = Instant::now();
    let mut planet_writer = PlanetWriter::new();
    PointGenerator::new()
        .take(n)
        .progress_count(n as u64)
        .filter(|random_point| !planet.is_on_land(random_point))
        .for_each(|random_point| planet_writer.add_point(&random_point));
    println!("generating points took {:?}", start.elapsed());

    let start = Instant::now();
    planet_writer.write(points_path);
    println!("writing points took {:?}", start.elapsed());
}
