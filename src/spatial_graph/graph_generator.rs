use std::io::Write;
use std::time::Instant;
use std::{collections::HashMap, f64::consts::PI, fs::File, io::BufWriter};

use indicatif::{ProgressBar, ProgressIterator};
use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::geometry::{
    meters_to_radians, radians_to_meter, Arc, CollisionDetection, Planet, Point,
};
use crate::spatial_partition::ConvecQuadrilateral;
use crate::spatial_partition::{PointSpatialPartition, PolygonSpatialPartition};

pub fn generate_network(num_nodes: u32, planet: &Planet, network_path: &str, planet_path: &str) {
    let radius = (4_000_000.0 * ((30_000.0 as f64).powi(2)) / num_nodes as f64).sqrt() * 1.0;
    println!("radius is {}", radius);

    let planet_grid = generate_planet_grid(planet);
    let points = generate_points(num_nodes, &planet_grid);

    let point_grid = generate_point_grid(&points);

    let start = Instant::now();
    let arcs = generate_arcs(&points, &point_grid, &planet_grid, radius);
    println!("took {:?} to generate arcs", start.elapsed());

    write_arcs_to_geojson(&arcs, planet_path);

    arcs_to_file(&arcs, &points, network_path);
}

fn write_arcs_to_geojson(arcs: &Vec<Arc>, planet_path: &str) {
    let mut out_planet = Planet::new();
    out_planet.arcs = arcs
        .iter()
        .map(|arc| arc._make_good_line())
        .flatten()
        .collect();
    out_planet.to_geojson_file(planet_path);
}

fn generate_points(how_many: u32, planet_grid: &PolygonSpatialPartition) -> Vec<Point> {
    println!("generating points");
    let mut points = Vec::new();

    let pb = ProgressBar::new(how_many as u64);
    while points.len() < how_many as usize {
        let point = Point::random();
        if !planet_grid.is_on_polygon(&point) {
            points.push(point);
            pb.inc(1);
        }
    }
    pb.finish_and_clear();

    points
}

fn generate_point_grid(points: &Vec<Point>) -> PointSpatialPartition {
    println!("generating point grid");
    let mut point_grid = PointSpatialPartition::new_root(10);
    points
        .iter()
        .progress()
        .for_each(|point| point_grid.add_point(point));
    point_grid
}

fn generate_planet_grid(planet: &Planet) -> PolygonSpatialPartition {
    println!("generating planet grid");
    let mut planet_grid = PolygonSpatialPartition::new(50);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));
    planet_grid.update_midpoints();
    planet_grid
}

fn arcs_to_file(arcs: &Vec<Arc>, points: &Vec<Point>, path: &str) {
    println!("enumerating points");
    let mut point_id_map = HashMap::new();
    for (i, point) in points.iter().enumerate().progress() {
        point_id_map.insert(point, i);
    }

    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(writer, "{}", points.len()).unwrap();
    writeln!(writer, "{}", arcs.len()).unwrap();
    println!("writing points to file");
    points.iter().progress().for_each(|point| {
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

    println!("writing arcs to file");
    arcs.iter().progress().for_each(|arc| {
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

fn generate_arcs(
    points: &Vec<Point>,
    point_grid: &PointSpatialPartition,
    planet_grid: &PolygonSpatialPartition,
    radius: f64,
) -> Vec<Arc> {
    println!("generating arcs");
    let arcs: Vec<_> = points
        .iter()
        .progress()
        .par_bridge()
        .map(|point| {
            vec![
                ur(point, radius),
                lr(point, radius),
                ll(point, radius),
                ul(point, radius),
            ]
            .iter()
            .filter_map(|polygon| {
                let mut local_points = point_grid.get_points(&polygon);

                local_points.sort_unstable_by(|x, y| {
                    Arc::new(point, x)
                        .central_angle()
                        .total_cmp(&Arc::new(point, y).central_angle())
                });

                // .get(1) is point
                if let Some(target) = local_points.get(1) {
                    let arc = Arc::new(point, &target);
                    if radians_to_meter(arc.central_angle()) <= radius {
                        return Some(arc);
                    }
                }

                None
            })
            .filter(|arc| !planet_grid.intersects_polygon(arc))
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    arcs
}

// works

fn ur(point: &Point, radius: f64) -> ConvecQuadrilateral {
    let cloned_point = point.clone();
    ConvecQuadrilateral::new(&vec![
        cloned_point,
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 1.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 0.0 / 4.0 * PI, meters_to_radians(radius)),
        cloned_point,
    ])
}

// works
fn lr(point: &Point, radius: f64) -> ConvecQuadrilateral {
    let cloned_point = point.clone();
    ConvecQuadrilateral::new(&vec![
        cloned_point,
        Point::destination_point(&point, 4.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 3.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 2.0 / 4.0 * PI, meters_to_radians(radius)),
        cloned_point,
    ])
}
fn ll(point: &Point, radius: f64) -> ConvecQuadrilateral {
    let cloned_point = point.clone();
    ConvecQuadrilateral::new(&vec![
        cloned_point,
        Point::destination_point(&point, 6.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 5.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 4.0 / 4.0 * PI, meters_to_radians(radius)),
        cloned_point,
    ])
}
fn ul(point: &Point, radius: f64) -> ConvecQuadrilateral {
    let cloned_point = point.clone();
    ConvecQuadrilateral::new(&vec![
        cloned_point,
        Point::destination_point(&point, 8.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 7.0 / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(&point, 6.0 / 4.0 * PI, meters_to_radians(radius)),
        cloned_point,
    ])
}
