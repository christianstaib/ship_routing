use std::{fs::File, io::BufWriter, io::Write};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    route::{RouteRequest, Routing},
    simple_algorithms::dijkstra::Dijkstra,
    Graph,
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
    /// Number of tests to be run
    #[arg(short, long)]
    number_of_tests: u32,
}

fn main() {
    let args = Args::parse();

    let graph = Graph::from_file(args.fmi_path.as_str());
    let dijkstra = Dijkstra::new(&graph);

    let mut writer = BufWriter::new(File::create("tests/data/fmi/test_cases.csv").unwrap());

    let mut rng = rand::thread_rng();
    let routes_requests: Vec<_> = (0..args.number_of_tests)
        .map(|_| RouteRequest {
            source: rng.gen_range(0..graph.nodes.len()) as u32,
            target: rng.gen_range(0..graph.nodes.len()) as u32,
        })
        .collect();

    let routes: Vec<_> = routes_requests
        .iter()
        .progress()
        .par_bridge()
        .map(|route_request| {
            let route_response = dijkstra.get_route(&route_request);
            let mut cost = "-".to_string();
            if let Some(route) = route_response {
                cost = route.cost.to_string();
            }
            format!("{},{},{}", route_request.source, route_request.target, cost)
        })
        .collect();

    routes
        .iter()
        .for_each(|route| writeln!(writer, "{}", route).unwrap());
    writer.flush().unwrap();
}
