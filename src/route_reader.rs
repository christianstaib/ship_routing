use std::{fs::File, io::BufRead, io::BufReader};

use crate::{fmi::Fmi, Point};

pub fn read_paths(in_path: &str, fmi: &Fmi) -> Vec<Vec<Point>> {
    println!("xx");
    let reader = BufReader::new(File::open(in_path).unwrap());
    let lines = reader.lines();
    let mut paths = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut path = Vec::new();
        let line: Vec<&str> = line.split(",").collect();
        println!("len line is {}", line.len());
        for id in line {
            let id: u32 = id.parse().unwrap();
            path.push(fmi.id_to_point(id));
        }
        paths.push(path);
    }

    paths
}
