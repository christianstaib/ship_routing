use std::{collections::HashMap, fs::File, io::BufWriter, io::Write};

use crate::planet::RawPlanet;

pub fn _write_coastline(planet: &RawPlanet, path: &str) {
    let mut writer = BufWriter::new(File::create(path).unwrap());
    planet.coastlines.iter().flatten().for_each(|node_id| {
        writeln!(
            writer,
            "{},{}",
            planet.nodes[node_id].lat, planet.nodes[node_id].lon
        )
        .unwrap();
    });

    writer.flush().unwrap();
}

pub fn _print_planet_statistic(planet: &RawPlanet) {
    println!("number of nodes: {}", planet.nodes.len());
    println!("number of coastlines: {}", planet.coastlines.len());
    println!(
        "min coastline len: {}",
        planet.coastlines.iter().map(|x| x.len()).min().unwrap()
    );
    println!(
        "max coastline len: {}",
        planet.coastlines.iter().map(|x| x.len()).max().unwrap()
    );
}

pub fn _check_diverging_ways(planet: &RawPlanet) {
    let mut last_map = HashMap::new();
    let mut first_map = HashMap::new();
    for (i, coastline) in planet.coastlines.iter().enumerate() {
        let last = coastline.last().unwrap().clone();
        if last_map.insert(last, i).is_some() {
            panic!("two way end on same node_id");
        };

        let first = coastline.first().unwrap().clone();
        if first_map.insert(first, i).is_some() {
            panic!("two ways start from same node_id")
        };
    }
}
