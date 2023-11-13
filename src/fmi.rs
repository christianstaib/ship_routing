use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use indicatif::ProgressIterator;

use crate::{Point, PointSpatialPartition};

pub struct Fmi {
    points_grid: PointSpatialPartition,
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

        Fmi {
            points_grid,
            points,
        }
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
}
