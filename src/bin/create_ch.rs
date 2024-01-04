use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::{
        contractor::Contractor,
        graph_cleaner::{remove_edge_to_self, removing_double_edges},
    },
    graph::Graph,
    naive_graph::NaiveGraph,
    route::RouteValidationRequest,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    fmi_path: String,
    /// Path of contracted_graph (output)
    #[arg(short, long)]
    contracted_graph: String,
    /// Path of .fmi file
    #[arg(short, long)]
    test_path: String,
}

fn main() {
    let args = Args::parse();

    let naive_graph = NaiveGraph::from_file(args.fmi_path.as_str());
    let mut graph = Graph::from_naive_graph(&naive_graph);
    removing_double_edges(&mut graph);
    remove_edge_to_self(&mut graph);

    let start = Instant::now();
    let contracted_graph = Contractor::get_graph_2(&graph);
    println!("Generating ch took {:?}", start.elapsed());

    let writer = BufWriter::new(File::create(args.contracted_graph).unwrap());
    bincode::serialize_into(writer, &contracted_graph).unwrap();

    let dijkstra = ChDijkstra::new(&contracted_graph);

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let mut times = Vec::new();
    for test in tests.iter().progress() {
        let before = Instant::now();
        let route = dijkstra.get_route(&test.request);
        times.push(before.elapsed());

        let mut cost = None;
        if let Some(route) = route {
            cost = Some(route.cost);
        }
        assert_eq!(cost, test.cost);
    }

    println!("all correct");
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
