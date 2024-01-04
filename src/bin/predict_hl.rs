use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressIterator};
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
    let contracted_graph: ContractedGraph = bincode::deserialize_from(reader).unwrap();

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

            vec![f_label, b_label]
        })
        .collect();
    println!(
        "Generating hl will take approx {:?}",
        Duration::from_secs_f32(
            start.elapsed().as_secs_f32() / tests.len() as f32
                * contracted_graph.graph.forward_edges.len() as f32
        )
    );

    let mut time_hl = Vec::new();
    tests
        .iter()
        .zip(label_size.iter())
        .progress()
        .for_each(|(test, labels)| {
            let start = Instant::now();
            let minimal_overlapp = labels[0].minimal_overlapp(&labels[1]);
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

    println!(
        "took {:?} per search",
        time_hl.iter().sum::<Duration>() / time_hl.len() as u32
    );

    println!(
        "avg label size is {:?} ",
        label_size
            .iter()
            .flatten()
            .map(|label| label.label.len())
            .sum::<usize>() as f32
            / label_size.iter().flatten().count() as f32
    );
}
