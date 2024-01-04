use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

use clap::Parser;
use osm_test::routing::hl::label::HubGraph;

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    contracted_graph_json: String,
    /// Path of .fmi file
    #[arg(short, long)]
    contracted_graph_bincode: String,
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let reader = BufReader::new(File::open(args.contracted_graph_json).unwrap());
    let contracted_graph: HubGraph = serde_json::from_reader(reader).unwrap();
    println!("took {:?} to read from serde", start.elapsed());

    let start = Instant::now();
    let writer = BufWriter::new(File::create(args.contracted_graph_bincode).unwrap());
    bincode::serialize_into(writer, &contracted_graph).unwrap();
    println!("took {:?} to write to bincode", start.elapsed());
}
