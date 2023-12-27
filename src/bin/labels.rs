use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::{
        contractor::Contractor,
        graph_cleaner::{remove_edge_to_self, removing_double_edges},
    },
    fast_graph::FastGraph,
    graph::Graph,
    hl::label::{self, Label},
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
    let contraced_graph = Contractor::get_graph_2(&graph);
    println!("contracting took {:?}", start.elapsed());

    let graph = FastGraph::from_graph(&contraced_graph.graph);
    let dijkstra = ChDijkstra::new(&graph, &contraced_graph.map);

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();

    let mut avg_label_size = 2;

    let mut times = Vec::new();
    for test in tests.iter().progress() {
        // let before = Instant::now();
        let route = dijkstra.get_route(&test.request);
        // times.push(before.elapsed());

        let max_depth = 0;
        let forward_label = dijkstra.get_backward_label(test.request.target, max_depth);
        avg_label_size += forward_label.len();
        let backward_label = dijkstra.get_backward_label(test.request.target, max_depth);
        avg_label_size += backward_label.len();

        let mut cost = None;
        if let Some(route) = route {
            cost = Some(route.cost);

            assert!(forward_label.contains_key(route.nodes.first().unwrap()));
            assert!(backward_label.contains_key(route.nodes.first().unwrap()));
            let forward_label = Label::new(&forward_label);
            let backward_label = Label::new(&backward_label);

            let before = Instant::now();
            assert!(forward_label.minimal_overlapp(&backward_label).is_some());
            times.push(before.elapsed());
            // println!("correct =)");
        }
        assert_eq!(cost, test.cost);
    }

    println!("all correct");
    println!(
        "avg label size {}",
        avg_label_size as f32 / (2 * times.len()) as f32
    );
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
