use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use indicatif::ProgressIterator;

use crate::{
    geometry::{Arc, Point},
    spatial_partition::PointSpatialPartition,
};

pub struct Fmi {
    points: Vec<Point>,
    arcs: Vec<Arc>,
}

impl Fmi {
    pub fn new(path: &str) -> Fmi {
        let mut points = Vec::new();
        let mut points_grid = PointSpatialPartition::new_root(25);
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines = reader.lines();
        let num_nodes = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let _num_arcs: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        for line in lines.take(num_nodes).progress_count(num_nodes as u64) {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let id: u32 = line.next().unwrap().parse().unwrap();
            let lat: f64 = line.next().unwrap().parse().unwrap();
            let lon: f64 = line.next().unwrap().parse().unwrap();
            let mut point = Point::from_coordinate(lat, lon);
            points_grid.add_point(&point);
            points.push(point);
        }

        let arcs = Vec::new();
        Fmi { points, arcs }
    }

    pub fn id_to_point(&self, id: u32) -> Point {
        let point = self.points[id as usize];

        point
    }

    pub fn read_paths(&self, in_path: &str) -> Vec<Vec<Point>> {
        let reader = BufReader::new(File::open(in_path).unwrap());
        let lines = reader.lines();
        let mut paths = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let mut path = Vec::new();
            let line: Vec<&str> = line.split(",").collect();
            for id in line {
                let id: u32 = id.parse().unwrap();
                path.push(self.id_to_point(id));
            }
            paths.push(path);
        }

        paths
    }
}
