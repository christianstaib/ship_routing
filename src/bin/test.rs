use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    route::{RouteValidationRequest, Routing},
    simple_algorithms::{
        a_star_with_distance::ASTarWithDistance, a_star_with_landmarks::AStarWithLandmarks,
        a_star_with_zero::AStarWithZero, bi_a_star_with_landmarks::BiAStarWithLandmarks,
        bi_a_star_with_zero::BiAStarWithZero,
    },
    Graph, NaiveGraph,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
    usize,
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

    let mut algorithms: Vec<(&str, Box<dyn Routing>)> = Vec::new();
    // algorithms.push((
    //     "a star with landmarks",
    //     Box::new(AStarWithLandmarks::new(&graph)),
    // ));
    // algorithms.push(("a star with zero", Box::new(AStarWithZero::new(&graph))));
    // algorithms.push((
    //     "a star with distance",
    //     Box::new(ASTarWithDistance::new(&graph)),
    // ));
    algorithms.push((
        "bidirectional a star with landmarks",
        Box::new(BiAStarWithLandmarks::new(&graph)),
    ));
    // algorithms.push((
    //     "dijkstra".to_string(),
    //     Box::new(dijkstra::Dijkstra::new(&graph)),
    // ));
    algorithms.push((
        "bidirectional a star with zero",
        Box::new(BiAStarWithZero::new(&graph)),
    ));

    let mut algorithms: Vec<_> = algorithms
        .iter()
        .map(|(name, algorithm)| (name, algorithm, Vec::new(), Vec::new()))
        .collect();

    let reader = BufReader::new(File::open("tests/data/fmi/test_cases.csv").unwrap());
    reader
        .lines()
        .take(args.number_of_tests as usize)
        .progress_count(args.number_of_tests as u64)
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let validation_request = RouteValidationRequest::from_str(line.as_str()).unwrap();
            let request = validation_request.request;

            for (name, routing_algorithm, times, scanned) in algorithms.iter_mut() {
                let before = Instant::now();
                let (route_response, route_data) = routing_algorithm.get_route(&request);
                times.push(before.elapsed());
                scanned.push(
                    route_data
                        .iter()
                        .map(|edpanded_ids| edpanded_ids.get_scanned_points().len())
                        .sum::<usize>(),
                );

                if let Some(route) = route_response {
                    assert!(
                        route.is_valid(&graph, &request),
                        "the returned route is not valid"
                    );
                    if let Some(true_cost) = validation_request.cost {
                        assert_eq!(
                            true_cost, route.cost,
                            "true cost is {} but \"{}\" got {}",
                            true_cost, name, route.cost
                        );
                    } else {
                        panic!("\"{}\" found a route when there shouldn't be one", name);
                    }
                } else {
                    assert!(validation_request.cost.is_none());
                }
            }
        });

    for (name, _, times, scanned) in algorithms.iter() {
        println!(
            "{:<40}: {:>4.3}ms, avgscan: {:>9}",
            name,
            (times.iter().sum::<Duration>() / times.len() as u32).as_secs_f64() * 1_000.0,
            scanned.iter().sum::<usize>() / scanned.len()
        );
    }
}
