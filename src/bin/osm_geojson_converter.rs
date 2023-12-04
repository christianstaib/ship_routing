use clap::Parser;
use osm_test::geometry::Planet;

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    input: String,
    /// Path of .fmi file
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let planet = Planet::from_osm_file(args.input.as_str());
    planet.to_geojson_file(args.output.as_str())
}
