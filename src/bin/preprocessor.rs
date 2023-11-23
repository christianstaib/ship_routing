<<<<<<< HEAD
use clap::Parser;
use osm_test::geometry::Planet;

use osm_test::spatial_graph::generate_network;

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    input: String,
    /// Path of .fmi file
    #[arg(short, long)]
    num_nodes: u32,
    /// Path of .fmi file
    #[arg(short, long)]
    output_network: String,
    /// Path of .fmi file
    #[arg(short, long)]
    output_geojson: String,
}

fn main() {
    let args = Args::parse();

    let planet = Planet::from_geojson_file(args.input.as_str()).unwrap();

    generate_network(
        args.num_nodes,
        &planet,
        args.output_network.as_str(),
        args.output_geojson.as_str(),
    );
=======
use osm_test::geometry::{OsmData, Planet};

use osm_test::spatial_graph::generate_network;

fn main() {
    const osm_file: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const NETWORK_PATH: &str = "test_4M.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();
    // let planet = Planet::from_osm_file(osm_file);

    generate_network(4_000_000, &planet, NETWORK_PATH, OUT_PLANET_PATH);
>>>>>>> 2955f64335bf35c4052004516c0c1078874dcb11
}
