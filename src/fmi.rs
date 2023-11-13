use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use indicatif::ProgressIterator;

use crate::{geometry::Point, grids::PointSpatialPartition};

pub struct Fmi {
    points: Vec<Point>,
}

impl Fmi {
    pub fn new(path: &str) -> Fmi {
        let mut points = Vec::new();
        let mut points_grid = PointSpatialPartition::new_root(25);
        let reader = BufReader::new(File::open(path).unwrap());
        let mut lines = reader.lines();
        let num_nodes: usize = lines
            .by_ref()
            .next()
            .unwrap()
            .unwrap()
            .parse::<_>()
            .unwrap();
        let _num_arcs: usize = lines
            .by_ref()
            .next()
            .unwrap()
            .unwrap()
            .parse::<_>()
            .unwrap();
        for line in lines.take(num_nodes).progress_count(num_nodes as u64) {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let id: u32 = line.next().unwrap().parse().unwrap();
            let lat: f64 = line.next().unwrap().parse().unwrap();
            let lon: f64 = line.next().unwrap().parse().unwrap();
            let mut point = Point::from_coordinate(lat, lon);
            point.id = Some(id);
            points_grid.add_point(&point);
            points.push(point);
        }

        Fmi { points }
    }

    pub fn id_to_point(&self, id: u32) -> Point {
        let point = self.points[id as usize];
        // self.points
        //     .iter()
        //     .find(|point| point.id.unwrap() == id)
        //     .unwrap()
        //     .clone()
        assert_eq!(point.id.unwrap(), id);

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
