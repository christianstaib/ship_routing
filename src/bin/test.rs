use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    graph::Graph,
    graph::NaiveGraph,
    route::{RouteResponse, RouteValidationRequest, Routing},
    simple_algorithms::{
        a_star_with_distance::ASTarWithDistance, a_star_with_landmarks::AStarWithLandmarks,
        a_star_with_zero::AStarWithZero, bi_a_star_with_landmarks::BiAStarWithLandmarks,
        bi_a_star_with_zero::BiAStarWithZero, dijkstra::Dijkstra,
    },
};
use std::{
    fs::File,
    io::BufReader,
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

    let algorithms: Vec<(&str, Box<dyn Routing>)> = vec![
        (
            "bidirectional a star with with zero",
            Box::new(BiAStarWithZero::new(&graph)),
        ),
        (
            "a star with landmarks",
            Box::new(AStarWithLandmarks::new(&graph)),
        ),
        ("dijkstra", Box::new(Dijkstra::new(&graph))),
        ("a star with zero", Box::new(AStarWithZero::new(&graph))),
        (
            "a star with distance",
            Box::new(ASTarWithDistance::new(&graph)),
        ),
        // (
        //     "bidirectional a star with landmarks",
        //     Box::new(BiAStarWithLandmarks::new(&graph)),
        // ),
    ];

    let mut algorithms: Vec<_> = algorithms
        .iter()
        .map(|(name, algorithm)| (name, algorithm, Vec::new(), Vec::new(), Vec::new()))
        .collect();

    let reader = BufReader::new(File::open("tests/data/fmi/tests.json").unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    tests
        .iter()
        .take(args.number_of_tests as usize)
        .progress_count(args.number_of_tests as u64)
        .for_each(|validation_request| {
            let request = &validation_request.request;

            for (_, routing_algorithm, times, scanned, legal) in algorithms.iter_mut() {
                let before = Instant::now();
                let response = routing_algorithm.get_route(&request);
                times.push(before.elapsed());
                scanned.push(
                    response
                        .data
                        .iter()
                        .map(|edpanded_ids| edpanded_ids.get_scanned_points().len())
                        .sum::<usize>(),
                );

                legal.push(response_is_legal(validation_request, response, &graph))
            }
        });

    for (name, _, times, scanned, legal) in algorithms.iter() {
        println!(
            "{:<40} legal? {:?} {:>4.3}ms, avgscan: {:>9}",
            name,
            legal.iter().all(|&b| b),
            (times.iter().sum::<Duration>() / times.len() as u32).as_secs_f64() * 1_000.0,
            scanned.iter().sum::<usize>() / scanned.len()
        );
    }
}

fn response_is_legal(
    request: &RouteValidationRequest,
    response: RouteResponse,
    graph: &Graph,
) -> bool {
    if let Some(route) = response.route {
        if !route.is_valid(graph, &request.request) {
            return false;
        }

        return request.cost == Some(route.cost);
    }
    return request.cost.is_none();
}
