use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

use clap::Parser;

use osm_test::routing::{
    ch::contractor::ContractedGraph, hl::label::HubGraph,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
use speedy::{Readable, Writable};

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    contracted_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    hub_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    hop_limit: u32,
}

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open(args.contracted_graph).unwrap());
    let contracted_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();

    let dijkstra = ChDijkstra::new(&contracted_graph);

    println!("starting hub label calculation");
    let start = Instant::now();
    let hub_graph = HubGraph::new(&dijkstra, args.hop_limit);
    println!("took {:?} to get hub graph", start.elapsed());

    println!("avg label size is {}", hub_graph.get_avg_label_size());

    let start = Instant::now();
    hub_graph.write_to_file("test.speedy").unwrap();
    println!("speedy: took {:?} to write pruned graph", start.elapsed());

    let start = Instant::now();
    let _ = HubGraph::read_from_file("test.speedy").unwrap();
    println!("speedy: took {:?} to read pruned graph", start.elapsed());

    let writer = BufWriter::new(File::create(args.hub_graph.as_str()).unwrap());
    serde_json::to_writer(writer, &hub_graph).unwrap();
    println!("wrote hub graph to file {}", args.hub_graph.as_str());
}
