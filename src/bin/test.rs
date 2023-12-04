use std::time::{Duration, Instant};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    dijkstra::fast_dijkstra::Dijkstra,
    route::{RouteRequest, Routing},
    Graph,
};
use rand::Rng;

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
    let number_nodes = graph.nodes.len();
    let dijkstra = Dijkstra::new(&graph);

    let mut rng = rand::thread_rng();

    let mut times = Vec::new();

    for _ in (0..args.number_of_tests).progress() {
        let route_request = RouteRequest {
            source: rng.gen_range(0..number_nodes) as u32,
            target: rng.gen_range(0..number_nodes) as u32,
        };
        let before = Instant::now();
        let route_response = dijkstra.get_route(&route_request);
        times.push(before.elapsed());
        if route_response.is_none() {
            println!("no route found");
        }
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
