use std::{fs::File, io::BufReader, time::Instant};

use clap::Parser;
use osm_test::routing::{
    ch::contractor::Contractor,
    fast_graph::FastGraph,
    graph::Graph,
    naive_graph::NaiveGraph,
    route::{RouteValidationRequest, Routing},
    simple_algorithms::bi_a_star_with_zero::BiAStarWithZero,
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

    let graph = FastGraph::new(contraced_graph.graph);

    let dijkstra = BiAStarWithZero::new(&graph);

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    for test in tests.iter().take(10) {
        let response = dijkstra.get_route(&test.request);
        let mut cost = None;
        if let Some(route) = response.route {
            cost = Some(route.cost);
        }
        println!("ist: {:?}, soll: {:?}", cost, test.cost);
    }
}
