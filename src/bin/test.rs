use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    dijkstra::{fast_dijkstra, no_option_and_no_expanded_dijkstra},
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
    let dijkstra1 = fast_dijkstra::Dijkstra::new(&graph);
    let dijkstra2 = no_option_and_no_expanded_dijkstra::Dijkstra::new(&graph);

    let mut times1 = Vec::new();
    let mut times2 = Vec::new();

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
            let route_response1 = dijkstra1.get_route(&route_request);
            times1.push(before.elapsed());
            let before = Instant::now();
            let route_response2 = dijkstra2.get_route(&route_request);
            times2.push(before.elapsed());
            if let Some(route) = route_response1 {
                if let Some(true_cost) = line[2].parse::<u32>().ok() {
                    assert_eq!(route.cost, true_cost, "wrong route cost");
                    let route2 = route_response2.unwrap();
                    assert_eq!(route.cost, route2.cost);
                } else {
                    panic!("couldn't parse cost");
                };
            } else {
                assert_eq!(line[2], "-");
            }
        });

    println!(
        "average time for Dijkstra1 is {:?}",
        times1.iter().sum::<Duration>() / times1.len() as u32
    );
    println!(
        "average time for Dijkstra2 is {:?}",
        times2.iter().sum::<Duration>() / times2.len() as u32
    );
}
