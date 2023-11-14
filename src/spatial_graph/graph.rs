use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use indicatif::ProgressIterator;

use crate::geometry::{Arc, Point};

pub struct Fmi {
    points: Vec<Point>,
    arcs: Vec<Arc>,
}

impl Fmi {
    pub fn new(path: &str) -> Fmi {
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines = reader.lines();

        let num_nodes = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let num_arcs = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let points: Vec<_> = lines
            .by_ref()
            .take(num_nodes)
            .enumerate()
            .progress_count(num_nodes as u64)
            .map(|(i, line)| {
                let line = line.unwrap();
                let mut line = line.split_whitespace();
                let id: u32 = line.next().unwrap().parse().unwrap();
                assert_eq!(id as usize, i);
                let lat = line.next().unwrap().parse().unwrap();
                let lon = line.next().unwrap().parse().unwrap();
                Point::from_coordinate(lat, lon)
            })
            .collect();

        let arcs = lines
            .by_ref()
            .take(num_arcs)
            .progress_count(num_arcs as u64)
            .map(|line| {
                let line = line.unwrap();
                let mut line = line.split_whitespace();
                let from: u32 = line.next().unwrap().parse().unwrap();
                let to: u32 = line.next().unwrap().parse().unwrap();
                let _cost: u32 = line.next().unwrap().parse().unwrap();
                Arc::new(&points[from as usize], &points[to as usize])
            })
            .collect();

        Fmi { points, arcs }
    }

    pub fn id_to_point(&self, id: u32) -> Point {
        let point = self.points[id as usize];

        point
    }

    pub fn convert_path(&self, path: &Vec<u32>) -> Vec<Point> {
        path.iter()
            .map(|&id| self.points[id as usize].clone())
            .collect()
    }

    pub fn read_paths(&self, in_path: &str) -> Vec<Vec<Point>> {
        let reader = BufReader::new(File::open(in_path).unwrap());
        let lines = reader.lines();
        let mut paths = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let line = line.split(",").map(|id| id.parse().unwrap()).collect();
            paths.push(self.convert_path(&line));
        }

        paths
    }
}
