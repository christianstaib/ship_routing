use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    usize,
};

use indicatif::ProgressIterator;

use crate::geometry::{radians_to_meter, Arc, Planet, Point};

pub struct Fmi {
    pub points: Vec<Point>,
    pub arcs: Vec<Arc>,
}

impl Fmi {
    pub fn from_file(path: &str) -> Fmi {
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

    pub fn to_file(&self, path: &str) {
        println!("enumerating points");
        let mut point_id_map = HashMap::new();
        for (i, point) in self.points.iter().enumerate().progress() {
            point_id_map.insert(point, i);
        }

        let mut writer = BufWriter::new(File::create(path).unwrap());
        writeln!(writer, "{}", self.points.len()).unwrap();
        writeln!(writer, "{}", self.arcs.len()).unwrap();
        println!("writing {} points to file", self.points.len());
        self.points.iter().progress().for_each(|point| {
            writeln!(
                writer,
                "{} {} {}",
                point_id_map.get(point).unwrap(),
                point.latitude(),
                point.longitude()
            )
            .unwrap();
        });
        writer.flush().unwrap();

        println!("writing {} arcs to file", self.arcs.len());
        self.arcs.iter().progress().for_each(|arc| {
            writeln!(
                writer,
                "{} {} {}",
                point_id_map.get(arc.from()).unwrap(),
                point_id_map.get(arc.to()).unwrap(),
                (radians_to_meter(arc.central_angle()) * 1.0) as u32
            )
            .unwrap();
        });
        writer.flush().unwrap();
    }

    pub fn to_planet(&self) -> Planet {
        let mut planet = Planet::new();
        planet.arcs = self
            .arcs
            .iter()
            .map(|arc| arc._make_good_line())
            .flatten()
            .collect();
        planet
    }

    pub fn nearest(&self, lon: f64, lat: f64) -> u32 {
        let point = Point::from_coordinate(lat, lon);
        self.points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let a = Arc::new(&a, &point);
                let b = Arc::new(&b, &point);
                a.central_angle().partial_cmp(&b.central_angle()).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap()
            .try_into()
            .unwrap()
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
}
