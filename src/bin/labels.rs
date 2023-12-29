use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{
    ch::{
        contractor::{ContractedGraph, Contractor},
        graph_cleaner::{remove_edge_to_self, removing_double_edges},
    },
    fast_graph::FastGraph,
    graph::Graph,
    hl::label::{self, HubGraph, Label},
    naive_graph::NaiveGraph,
    route::RouteValidationRequest,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use warp::filters::trace::request;

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
    // removing_double_edges(&mut graph);
    // remove_edge_to_self(&mut graph);

    // let start = Instant::now();
    // let contraced_graph = Contractor::get_graph_2(&graph);
    // println!("contracting took {:?}", start.elapsed());

    // {
    //     let writer = BufWriter::new(File::create("contraced_graph.json").unwrap());
    //     serde_json::to_writer(writer, &contraced_graph).unwrap();
    // }

    let reader = BufReader::new(File::open("contraced_graph.json").unwrap());
    let contraced_graph: ContractedGraph = serde_json::from_reader(reader).unwrap();

    let shortcuts = &contraced_graph.map.into_iter().collect();

    let graph = FastGraph::from_graph(&contraced_graph.graph);
    let dijkstra = ChDijkstra::new(&graph, shortcuts);

    let reader = BufReader::new(File::open(args.test_path.as_str()).unwrap());
    let tests: Vec<RouteValidationRequest> = serde_json::from_reader(reader).unwrap();
    let start = Instant::now();
    let f_labels: Vec<_> = tests
        .iter()
        .progress()
        .par_bridge()
        .map(|test| dijkstra.get_forward_label(test.request.source, 0))
        .collect();
    let b_labels: Vec<_> = tests
        .iter()
        .progress()
        .par_bridge()
        .map(|test| dijkstra.get_backward_label(test.request.source, 0))
        .collect();
    println!("took {:?} per node", start.elapsed() / tests.len() as u32);

    let start = Instant::now();
    let hub_graph = HubGraph::new(&dijkstra, 2);
    println!("getting labels took {:?}", start.elapsed());

    {
        let writer = BufWriter::new(File::create("hub_graph.json").unwrap());
        serde_json::to_writer(writer, &hub_graph).unwrap();
    }

    let mut time_hl = Vec::new();
    let mut label_creation = Vec::new();
    tests.iter().progress().for_each(|test| {
        // let before = Instant::now();
        // let route = dijkstra.get_route(&test.request);
        // times.push(before.elapsed());

        let start = Instant::now();
        let forward_label = dijkstra.get_forward_label(test.request.source, 2);
        let backward_label = dijkstra.get_backward_label(test.request.target, 2);
        let forward_label = Label::new(&forward_label);
        let backward_label = Label::new(&backward_label);
        label_creation.push(start.elapsed());
        // let minimal_overlapp = forward_label.minimal_overlapp(&backward_label);

        if let Some(true_cost) = test.cost {
            let start = Instant::now();
            let minimal_overlapp = forward_label.minimal_overlapp(&backward_label).unwrap();
            time_hl.push(start.elapsed());
            let my_cost = minimal_overlapp.cost;
            assert_eq!(my_cost, true_cost);
        }
    });

    println!(
        "took {:?} per search",
        time_hl.iter().sum::<Duration>() / time_hl.len() as u32
    );
    println!(
        "took {:?} per label creation",
        label_creation.iter().sum::<Duration>() / (2 * label_creation.len()) as u32
    );
}
