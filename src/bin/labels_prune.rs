use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::contractor::ContractedGraph, hl::label::HubGraph, route::RouteValidationRequest,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
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

    let start = Instant::now();
    let reader = BufReader::new(File::open(args.hub_graph).unwrap());
    let mut hub_graph: HubGraph = serde_json::from_reader(reader).unwrap();
    println!("took {:?} to read graph", start.elapsed());

    println!("avg label size is {}", hub_graph.get_avg_label_size());

    let start = Instant::now();
    hub_graph.write_to_file("test.speedy").unwrap();
    println!("speedy: took {:?} to write pruned graph", start.elapsed());

    let start = Instant::now();
    let _ = HubGraph::read_from_file("test.speedy").unwrap();
    println!("speedy: took {:?} to read pruned graph", start.elapsed());

    hub_graph.prune();

    println!("avg label size is {}", hub_graph.get_avg_label_size());

    let start = Instant::now();
    let writer = BufWriter::new(File::create(args.pruned_hub_graph).unwrap());
    serde_json::to_writer(writer, &hub_graph).unwrap();
    println!("took {:?} to write pruned graph", start.elapsed());

    let mut time_hl = Vec::new();
    tests.iter().progress().for_each(|test| {
        let start = Instant::now();
        let minimal_overlapp = hub_graph.get_route(&test.request);
        time_hl.push(start.elapsed());

        if let Some(true_cost) = test.cost {
            let my_cost = minimal_overlapp.unwrap().cost;
            assert_eq!(
                my_cost, true_cost,
                "should be {} but is {}",
                true_cost, my_cost
            );
        } else {
            assert!(minimal_overlapp.is_none());
        }
    });

    println!("all correct");

    println!(
        "took {:?} per search",
        time_hl.iter().sum::<Duration>() / time_hl.len() as u32
    );
}
