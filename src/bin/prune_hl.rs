use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{hl::label::HubGraph, route::RouteValidationRequest};
use speedy::{Readable, Writable};

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    hub_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    pruned_hub_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    test_path: String,
}

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let reader = BufReader::new(File::open(args.hub_graph).unwrap());
    let mut hub_graph: HubGraph = bincode::deserialize_from(reader).unwrap();

    hub_graph.prune();

    let start = Instant::now();
    let writer = BufWriter::new(File::create(args.pruned_hub_graph).unwrap());
    serde_json::to_writer(writer, &hub_graph).unwrap();
    println!("took {:?} to write pruned graph", start.elapsed());

    let mut time_hl = Vec::new();
    tests.iter().progress().for_each(|test| {
        let start = Instant::now();
        let cost = hub_graph.get_cost(&test.request);
        time_hl.push(start.elapsed());

        assert_eq!(cost, test.cost);
    });

    println!("all correct");

    println!(
        "took {:?} per search",
        time_hl.iter().sum::<Duration>() / time_hl.len() as u32
    );
}
