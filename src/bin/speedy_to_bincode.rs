use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

use clap::Parser;

use osm_test::routing::{
    ch::contractor::ContractedGraph, hl::label::HubGraph,
    simple_algorithms::ch_bi_dijkstra::ChDijkstra,
};
use speedy::Readable;

fn main() {
    let start = Instant::now();
    let contracted_graph: ContractedGraph = ContractedGraph::read_from_file("test.speedy").unwrap();
    println!("took {:?} to read from speedy", start.elapsed());

    let start = Instant::now();
    let writer = BufWriter::new(File::create("test.bincode").unwrap());
    bincode::serialize_into(writer, &contracted_graph).unwrap();
    println!("took {:?} to write to bincode", start.elapsed());
}
