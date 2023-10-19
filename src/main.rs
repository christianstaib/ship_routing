use std::time::Instant;

use indicatif::ProgressIterator;

use crate::geojson_writer::GeoJsonWriter;
use crate::planet::Planet;
use crate::planet_elements::new_planet::NewPlanet;
use crate::point_generator::PointGenerator;

pub mod geojson_writer;
mod planet;
pub mod planet_elements;
pub mod point_generator;

fn main() {
    let pbf_path = "data/planet-coastlinespbf-cleanedosmpbf.osm.pbf";
    //let pbf_path = "data/antarctica-latest.osm.pbf";
    let planet_path = "data/geojson/planet.geo.json";
    let points_path = "data/geojson/points.geo.json";
    let n = 100;

    let start = Instant::now();
    let planet = Planet::from_path(pbf_path);
    println!("loading pbf file took {:?}", start.elapsed());

    let start = Instant::now();
    planet.to_file(planet_path);
    println!("writing file took {:?}", start.elapsed());

    let planet = NewPlanet::from_planet(&planet);

    let start = Instant::now();
    let mut geojson_writer = GeoJsonWriter::new(points_path);
    PointGenerator::new()
        .take(n)
        .progress_count(n as u64)
        .filter(|random_point| !planet.is_on_land_winding(random_point))
        .for_each(|random_point| geojson_writer.add_point(&random_point));
    geojson_writer.flush();
    let end = start.elapsed();
    println!(
        "generating points took {:?} which is {:?} per point",
        end,
        end / n as u32
    );
}
