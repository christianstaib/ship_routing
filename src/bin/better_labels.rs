use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::{
        contractor::{ContractedGraph, Contractor},
        graph_cleaner::{remove_edge_to_self, removing_double_edges},
    },
    fast_graph::FastGraph,
    graph::Graph,
    hl::label::{self, HubGraph, Label},
    naive_graph::NaiveGraph,
    route::RouteValidationRequest,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};
use warp::filters::trace::request;

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    fmi_path: String,
    /// Path of .fmi file
    #[arg(short, long)]
    test_path: String,
}

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open("contraced_graph_network.json").unwrap());
    let contraced_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();
    let num_nodes = contraced_graph.graph.forward_edges.len();

    let shortcuts = &contraced_graph.map.into_iter().collect();

    let graph = FastGraph::from_graph(&contraced_graph.graph);
    let dijkstra = ChDijkstra::new(&graph, shortcuts);

    let mut rng = rand::thread_rng();
    let n = 5_000;
    let hop_limit = 1;

    let rand_nums: Vec<_> = (0..n)
        .progress()
        .map(|_| rng.gen_range(0..num_nodes))
        .collect();

    let start = Instant::now();
    let all_labels_f: Vec<_> = rand_nums
        .iter()
        .progress()
        .par_bridge()
        .map(|&test| dijkstra.get_forward_label(test as u32, hop_limit))
        .collect();
    let label = start.elapsed();

    let start = Instant::now();
    let all_labels_b: Vec<_> = rand_nums
        .iter()
        .progress()
        .par_bridge()
        .map(|&test| dijkstra.get_backward_label(test as u32, hop_limit))
        .collect();
    let label_vec = start.elapsed();

    println!(
        "there are {:?} nodes in an average label",
        (all_labels_f.iter().flatten().count() + all_labels_b.iter().flatten().count())
            / (2 * n as usize)
    );
    println!("took {:?} per label creation", label / n);

    println!("took {:?} per label creation vec", label_vec / n);
}
