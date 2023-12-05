use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    dijkstra::no_option_and_no_expanded_dijkstra::Dijkstra,
    route::{RouteRequest, Routing},
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
    let dijkstra = Dijkstra::new(&graph);

    let mut times = Vec::new();

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
            let before = Instant::now();
            let route_response = dijkstra.get_route(&route_request);
            times.push(before.elapsed());
            if let Some(route) = route_response {
                if let Some(true_cost) = line[2].parse::<u32>().ok() {
                    assert_eq!(route.cost, true_cost, "wrong route cost");
                } else {
                    panic!("couldn't parse cost");
                };
            } else {
                assert_eq!(line[2], "-");
            }
        });

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
