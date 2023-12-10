use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
    usize,
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    route::{RouteRequest, Routing},
    simple_algorithms::*,
    Graph, NaiveGraph,
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

    let naive_graph = NaiveGraph::from_file(args.fmi_path.as_str());

    let graph = Graph::new(naive_graph);

    let mut algorithms: Vec<(String, Box<dyn Routing>, Vec<Duration>)> = Vec::new();
    // // algorithms.push((
    // //     "a start".to_string(),
    // //     Box::new(a_star::Dijkstra::new(&graph)),
    // //     Vec::new(),
    // // ));
    // // algorithms.push((
    // //     "bidirectional a start".to_string(),
    // //     Box::new(bidirectional_a_star::Dijkstra::new(&graph)),
    // //     Vec::new(),
    // // ));
    // // algorithms.push((
    // //     "landmark a star".to_string(),
    // //     Box::new(landmark::Dijkstra::new(&graph)),
    // //     Vec::new(),
    // // ));
    algorithms.push((
        "bidirectional landmark a star".to_string(),
        Box::new(bidirectional_landmark::BiLandmark::new(&graph)),
        Vec::new(),
    ));
    // algorithms.push((
    //     "dijkstra".to_string(),
    //     Box::new(dijkstra::Dijkstra::new(&graph)),
    //     Vec::new(),
    // ));
    // algorithms.push((
    //     "bidirectional dijkstra".to_string(),
    //     Box::new(bidirectional_dijkstra::BiDijkstra::new(&graph)),
    //     Vec::new(),
    // ));

    let reader = BufReader::new(File::open("tests/data/fmi/test_cases.csv").unwrap());
    reader
        .lines()
        .take(args.number_of_tests as usize)
        .progress_count(args.number_of_tests as u64)
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let line: Vec<_> = line.split(',').collect();
            let request = RouteRequest {
                source: line[0].parse().unwrap(),
                target: line[1].parse().unwrap(),
            };

            for (name, routing_algorithm, times) in algorithms.iter_mut() {
                let before = Instant::now();
                let route_response = routing_algorithm.get_route(&request);
                times.push(before.elapsed());
                if let Some(route) = route_response {
                    route.is_valid(&graph, &request);
                    assert_eq!(route.nodes.first().unwrap(), &request.source);
                    assert_eq!(route.nodes.last().unwrap(), &request.target);
                    let true_cost = line[2].parse::<u32>().unwrap();
                    assert_eq!(
                        true_cost, route.cost,
                        "true cost is {} but \"{}\" got {}",
                        true_cost, name, route.cost
                    );
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
