use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ParallelProgressIterator;
use osm_test::routing::{
    ch::contractor::ContractedGraph, hl::label::Label, route::RouteValidationRequest,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    contracted_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    test_path: String,
    /// Path of .fmi file
    #[arg(short, long)]
    hop_limit: u32,
}

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open(args.contracted_graph).unwrap());
    let contracted_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let dijkstra = ChDijkstra::new(&contracted_graph);

    let start = Instant::now();
    let label_size: Vec<_> = tests
        .par_iter()
        .progress()
        .map(|test| {
            let f_label =
                Label::new(&dijkstra.get_forward_label(test.request.source, args.hop_limit));
            let b_label =
                Label::new(&dijkstra.get_backward_label(test.request.target, args.hop_limit));
            // f_label.prune_forward(test.request.source, &dijkstra);
            // b_label.prune_backward(test.request.target, &dijkstra);

            vec![f_label.label.len(), b_label.label.len()]
        })
        .flatten()
        .collect();
    println!(
        "took {:?}, which is {:?} for all nodes",
        start.elapsed(),
        Duration::from_secs_f32(
            start.elapsed().as_secs_f32() / tests.len() as f32
                * contracted_graph.graph.forward_edges.len() as f32
        )
    );

    println!(
        "avg label size is {:?} ",
        label_size.iter().sum::<usize>() as f32 / label_size.len() as f32
    );
}
