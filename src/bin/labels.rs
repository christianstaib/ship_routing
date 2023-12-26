use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::contractor::{ContractedGraph, Contractor},
    fast_graph::{FastEdgeAccess, FastGraph},
    graph::Graph,
    naive_graph::NaiveGraph,
    route::{RouteRequest, RouteValidationRequest, Routing},
    simple_algorithms::{
        bi_a_star_with_zero::BiAStarWithZero, ch_bi_dijkstra::ChDijkstra, dijkstra::Dijkstra,
    },
};
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

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

    let naive_graph = NaiveGraph::from_file(args.fmi_path.as_str());
    let graph = Graph::from_naive_graph(&naive_graph);

    let cloned_graph = FastGraph::new(&naive_graph);
    let true_dijkstra = Dijkstra::new(&cloned_graph);

    // let mut contractor = Contractor::new(graph);
    // let start = Instant::now();
    // println!("start contrating");
    // contractor.contract();
    // let contraced_graph = contractor.get_graph();
    // println!("contracting took {:?}", start.elapsed());

    let reader = BufReader::new(File::open("graph.json").unwrap());
    let contraced_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();

    println!("there are {} nodes", naive_graph.nodes.len());
    println!("there are {} shortcuts", contraced_graph.map.len());
    println!(
        "there are {} forward edges",
        contraced_graph.graph.forward_edges.iter().flatten().count()
    );
    println!(
        "there are {} backward edges",
        contraced_graph
            .graph
            .backward_edges
            .iter()
            .flatten()
            .count()
    );

    let num_nodes = contraced_graph.graph.backward_edges.len() as u32;

    let forward_edges = FastEdgeAccess::new(
        &contraced_graph
            .graph
            .forward_edges
            .into_iter()
            .flatten()
            .collect(),
    );
    let backward_edges = FastEdgeAccess::new(
        &contraced_graph
            .graph
            .backward_edges
            .into_iter()
            .flatten()
            .map(|edge| edge.get_inverted())
            .collect(),
    );

    let graph = FastGraph {
        num_nodes,
        forward_edges,
        backward_edges,
    };

    let ch_dijkstra = ChDijkstra::new(&graph);

    // let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    // let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let mut rng = rand::thread_rng();

    let n = 100;
    let mut times = Vec::new();
    let before = Instant::now();
    (0..n).take(n).progress().for_each(|_| {
        let source = rng.gen_range(0..num_nodes);
        let target = rng.gen_range(0..num_nodes);
        let forward_label = ch_dijkstra.get_forward_label(source);
        let mut forward_label: Vec<(u32, u32)> = forward_label.into_iter().collect();
        forward_label.sort_unstable_by_key(|(id, _)| *id);
        let forward_id = forward_label.iter().map(|(id, _)| id).collect::<Vec<_>>();
        let forward_cost = forward_label
            .iter()
            .map(|(_, cost)| cost)
            .collect::<Vec<_>>();

        let backward_label = ch_dijkstra.get_backward_label(target);
        let backward_label: Vec<(u32, u32)> = backward_label.into_iter().collect();
    });
    times.push(before.elapsed());

    println!("all correct");
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / n as u32
    );
}
