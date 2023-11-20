use std::time::{Duration, Instant};

use clap::Parser;
use indicatif::ProgressIterator;
use osm_test::routing::{Dijkstra, Graph};
use rand::Rng;

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

    let graph = Graph::from_file(args.fmi_path.as_str());
    let number_nodes = graph.nodes.len();
    let dijkstra = Dijkstra::new(&graph);

    let mut rng = rand::thread_rng();

    let mut times = Vec::new();

    for _ in (0..1_000).progress() {
        let source = rng.gen_range(0..number_nodes) as u32;
        let target = rng.gen_range(0..number_nodes) as u32;
        let before = Instant::now();
        let (_, cost) = dijkstra.dijkstra(source, target);
        times.push(before.elapsed());
        if cost == u32::MAX {
            println!("no route found");
        }
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
