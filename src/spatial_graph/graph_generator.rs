use std::io::Write;

use std::time::Instant;
use std::{collections::HashMap, f64::consts::PI, fs::File, io::BufWriter};

use indicatif::{ProgressBar, ProgressIterator};
use rayon::prelude::*;

use crate::geometry::{
    meters_to_radians, radians_to_meter, Arc, CollisionDetection, Contains, Planet, Point,
    PointGenerator,
};
use crate::spatial_partition::ConvecQuadrilateral;
use crate::spatial_partition::{PointSpatialPartition, PolygonSpatialPartition};

use super::Fmi;

pub fn generate_network(num_nodes: u32, planet: &Planet, network_path: &str, planet_path: &str) {
    let start = Instant::now();
    let planet_grid = generate_planet_grid(planet);
    let mut points = generate_points(num_nodes, &planet_grid);

    // let filter = Planet::from_geojson_file("filter.geojson").unwrap();
    // points.retain(|p| filter.polygons[0].contains(p));

    println!("took {:?}", start.elapsed());
    let point_grid = generate_point_grid(&points);
    let arcs = generate_arcs(&points, &point_grid, &planet_grid, 30_000.0);

    let fmi = Fmi { points, arcs };
    fmi.to_file(network_path);
    fmi.to_planet().to_geojson_file(planet_path);
}

fn generate_points(how_many: u32, planet_grid: &PolygonSpatialPartition) -> Vec<Point> {
    println!("generating points");
    PointGenerator::new()
        .filter(|point| point.latitude() >= -82.0)
        .filter(|point| !planet_grid.is_on_polygon(point))
        .take(how_many as usize)
        .progress_count(how_many as u64)
        .collect()
}

fn generate_point_grid(points: &Vec<Point>) -> PointSpatialPartition {
    println!("generating point grid");
    let mut point_grid = PointSpatialPartition::new_root(10);
    point_grid.add_points(points);
    point_grid
}

fn generate_planet_grid(planet: &Planet) -> PolygonSpatialPartition {
    println!("generating planet grid");
    let mut planet_grid = PolygonSpatialPartition::new(50);
    planet_grid.add_polygons(&planet.polygons);
    planet_grid
}

fn generate_arcs(
    points: &Vec<Point>,
    point_grid: &PointSpatialPartition,
    planet_grid: &PolygonSpatialPartition,
    radius: f64,
) -> Vec<Arc> {
    println!("generating arcs");
    points
        .iter()
        .progress()
        .par_bridge()
        .map(|point| {
            vec![
                ur(point, radius, 2.0),
                ur(point, radius, 4.0),
                ur(point, radius, 6.0),
                ur(point, radius, 8.0),
            ]
            .iter()
            .filter_map(|polygon| {
                let mut local_points = point_grid.get_points(&polygon);

                local_points.sort_unstable_by(|x, y| {
                    Arc::new(point, x)
                        .central_angle()
                        .total_cmp(&Arc::new(point, y).central_angle())
                });

                // if first point is point, take second
                let mut idx = 0;
                if let Some(first_point) = local_points.get(idx) {
                    if first_point == point {
                        idx += 1;
                    }
                }
                if let Some(target) = local_points.get(idx) {
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
        .collect()
}

fn ur(point: &Point, radius: f64, start: f64) -> ConvecQuadrilateral {
    let cloned_point = point.clone();
    ConvecQuadrilateral::new(&vec![
        cloned_point,
        Point::destination_point(&point, (start) / 4.0 * PI, meters_to_radians(radius)),
        Point::destination_point(
            &point,
            (start - 1.0) / 4.0 * PI,
            meters_to_radians((radius.powi(2) + radius.powi(2)).sqrt()),
        ),
        Point::destination_point(&point, (start - 2.0) / 4.0 * PI, meters_to_radians(radius)),
        cloned_point,
    ])
}
