use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::{
    geometry::{radians_to_meter, Arc},
    routing::{
        route::{RouteRequest, Routing},
        simple_algorithms::dijkstra,
        Graph, NaiveGraph,
    },
};
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};

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
    let routing_algorithm = dijkstra::Dijkstra::new(&graph);

    println!("check all arcs");
    (0..graph.nodes.len())
        .progress()
        .par_bridge()
        .for_each(|source| {
            let edges = graph.outgoing_edges(source as u32);
            for edge in edges {
                let h = radians_to_meter(
                    Arc::new(
                        &graph.nodes[source as usize],
                        &graph.nodes[edge.target as usize],
                    )
                    .central_angle(),
                ) as u32;
                assert!(h <= edge.cost);
            }

            let edges = graph.incoming_edges(source as u32);
            for edge in edges {
                let h = radians_to_meter(
                    Arc::new(
                        &graph.nodes[source as usize],
                        &graph.nodes[edge.target as usize],
                    )
                    .central_angle(),
                ) as u32;
                assert!(h <= edge.cost);
            }
        });

    println!("check random nodes");
    (0..args.number_of_tests)
        .progress_count(args.number_of_tests as u64)
        .par_bridge()
        .for_each(|_| {
            let mut rng = rand::thread_rng();

            let request = RouteRequest {
                source: rng.gen_range(0..graph.nodes.len()) as u32,
                target: rng.gen_range(0..graph.nodes.len()) as u32,
            };

            let route_response = routing_algorithm.get_route(&request);
            if let Some(route) = route_response {
                route.is_valid(&graph, &request);
                let h = radians_to_meter(
                    Arc::new(
                        &graph.nodes[request.source as usize],
                        &graph.nodes[request.target as usize],
                    )
                    .central_angle(),
                ) as u32;
                assert!(h <= route.cost);
            }
        });
}
