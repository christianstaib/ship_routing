use clap::Parser;
use osm_test::routing::{ch::contrator::Contractor, graph::Graph, naive_graph::NaiveGraph};

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    fmi_path: String,
}

fn main() {
    let args = Args::parse();

    let mut naive_graph = NaiveGraph::from_file(args.fmi_path.as_str());
    naive_graph.make_bidirectional();
    let graph = Graph::from_naive_graph(&naive_graph);
    let mut contractor = Contractor::new(graph);

    println!("start contrating");
    contractor.contract();

    println!("there are {:?} shortcuts", contractor.shortcuts.len());
}
