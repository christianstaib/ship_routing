use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    route::{RouteRequest, Routing},
    simple_algorithms::{bidirectional_dijkstra, dijkstra},
    Graph,
};

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
    let mut algorithms: Vec<(String, Box<dyn Routing>, Vec<Duration>)> = Vec::new();
    algorithms.push((
        "dijkstra".to_string(),
        Box::new(dijkstra::Dijkstra::new(&graph)),
        Vec::new(),
    ));
    algorithms.push((
        "bidirectional dijkstra".to_string(),
        Box::new(bidirectional_dijkstra::Dijkstra::new(&graph)),
        Vec::new(),
    ));

    let reader = BufReader::new(File::open("tests/data/fmi/test_cases.csv").unwrap());
    reader
        .lines()
        .take(args.number_of_tests as usize)
        .progress_count(args.number_of_tests as u64)
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let line: Vec<_> = line.split(',').collect();
            let route_request = RouteRequest {
                source: line[0].parse().unwrap(),
                target: line[1].parse().unwrap(),
            };
            for (_, routing_algorithm, times) in algorithms.iter_mut() {
                let before = Instant::now();
                let route_response = routing_algorithm.get_route(&route_request);
                times.push(before.elapsed());
                if let Some(route) = route_response {
                    let true_cost = line[2].parse::<u32>().unwrap();
                    assert_eq!(route.cost, true_cost, "wrong route cost");
                } else {
                    assert_eq!(line[2], "-");
                }
            }
        });

    for (name, _, times) in algorithms.iter() {
        println!(
            "average time for {:?} is {:?}",
            name,
            times.iter().sum::<Duration>() / times.len() as u32
        );
    }
}
