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
    route::{RouteValidationRequest, Routing},
    simple_algorithms::{bi_a_star_with_zero::BiAStarWithZero, ch_bi_dijkstra::ChDijkstra},
};

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

    let mut contractor = Contractor::new(graph);
    let start = Instant::now();
    println!("start contrating");
    contractor.contract();
    let contraced_graph = contractor.get_graph();
    println!("contracting took {:?}", start.elapsed());

    // let reader = BufReader::new(File::open("graph.json").unwrap());
    // let contraced_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();

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

    let shortcuts = contraced_graph.map.iter().cloned().collect();

    let dijkstra = ChDijkstra::new(&graph, &shortcuts);

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let mut times = Vec::new();
    for _ in (0..100).progress() {
        for test in tests.iter().take(10) {
            let before = Instant::now();
            let route = dijkstra.get_route(&test.request);
            times.push(before.elapsed());

            let mut cost = None;
            if let Some(route) = route {
                cost = Some(route.cost);
            }
            assert_eq!(cost, test.cost);
        }
    }

    println!("all correct");
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
