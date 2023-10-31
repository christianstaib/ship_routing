use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Instant,
};

use indicatif::ProgressIterator;
use osm_test::{point_generator::PointGenerator, Planet, Point, Polygon};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    // _test_inside_point();
    // _get_normal();
    // _generate_points();
    // test_clipping();
    // test_clipping_two();
    test_clipping_three();
}

fn test_clipping_three() {
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/test_clipping3.geojson";
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Arc::new(Mutex::new(Planet::new()));

    let lon_min = -6;
    let lon_max = 6;
    let lat_min = 49;
    let lat_max = 61;
    let sw = Point::from_geodetic(lat_min as f64, lon_min as f64);
    let ne = Point::from_geodetic(lat_max as f64, lon_max as f64);
    let clipping_polygon = generate_rectangle(&sw, &ne);
    let polygons: Vec<Polygon> = planet
        .polygons
        .iter()
        .map(|polygon| polygon.clip(&clipping_polygon))
        .flatten()
        .collect();
    (-5..5).progress().par_bridge().for_each(|lon| {
        (50..60).for_each(|lat| {
            println!("lat {} lon {}", lat, lon);
            let sw = Point::from_geodetic(lat as f64, lon as f64);
            let ne = Point::from_geodetic((lat + 1) as f64, (lon + 1) as f64);
            let clipping_polygon = generate_rectangle(&sw, &ne);
            out_planet.lock().unwrap().polygons.extend(
                polygons
                    .iter()
                    .map(|polygon| polygon.clip(&clipping_polygon))
                    .flatten(),
            );
            // out_planet.polygons.push(polygon);
        })
    });

    out_planet.lock().unwrap().to_file(OUT_PLANET_PATH);
}

fn generate_rectangle(sw: &Point, ne: &Point) -> Polygon {
    assert!(sw.lat() < ne.lat());
    assert!(sw.lon() < ne.lon());

    let mut outline = vec![
        Point::from_geodetic(sw.lat(), sw.lon()),
        Point::from_geodetic(sw.lat(), ne.lon()),
        Point::from_geodetic(ne.lat(), ne.lon()),
        Point::from_geodetic(ne.lat(), sw.lon()),
        Point::from_geodetic(sw.lat(), sw.lon()),
    ];
    outline.dedup();
    assert!(outline.len() >= 4);
    Polygon::new(outline)
}

fn test_clipping_two() {
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/test_clipping.geojson";
    let mut out_planet = Planet::new();

    let sw = Point::from_geodetic(0.0, -2.0);
    let ne = Point::from_geodetic(1.0, 2.0);
    let polygon = generate_rectangle(&sw, &ne);
    out_planet.polygons.push(polygon.clone());

    let sw = Point::from_geodetic(-1.0, -1.0);
    let ne = Point::from_geodetic(2.0, 0.0);
    let clipping_polygon = generate_rectangle(&sw, &ne);
    out_planet.polygons.push(clipping_polygon.clone());

    out_planet.polygons.extend(polygon.clip(&clipping_polygon));

    out_planet.to_file(OUT_PLANET_PATH);
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/test_clipping.geojson";

    // let planet = Planet::from_osm(PLANET_PATH);
    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = planet.clone(); //Planet::new();
    planet
        .polygons
        .sort_by_key(|polygon| -1 * polygon.outline.len() as isize);
    // planet.polygons.iter().for_each(|polygon| {
    //     let point = polygon.get_inside_point_better(&mut out_planet);
    //     out_planet.points.push(point);
    // });
    // let polygon = planet.polygons[0].clone();
    // out_planet.polygons.push(polygon.clone());
    // let point = polygon.get_inside_point_better(&mut out_planet);
    // out_planet.points.push(point);

    let sw = Point::from_geodetic(48.0, -3.0);
    let ne = Point::from_geodetic(49.0, -2.0);

    let clipping_polygon = generate_rectangle(&sw, &ne);
    out_planet.polygons.push(clipping_polygon.clone());

    planet.polygons.iter().for_each(|polygon| {
        let clipped = polygon.intersections_polygon(&clipping_polygon);
        out_planet
            .points
            .extend(clipped.iter().map(|x| x.2.inner()));
        let clipped = polygon.clip(&clipping_polygon);
        out_planet.polygons.extend(clipped);
        // let out: Vec<Point> = polygon
        //     .intersections_polygon(&clipping_polygon)
        //     .iter()
        //     .filter_map(|x| match x.2 {
        //         osm_test::PointClassification::OutIntersection(p) => Some(p),
        //         _ => None,
        //     })
        //     .collect();
        // let inter: Vec<Point> = polygon
        //     .intersections_polygon(&clipping_polygon)
        //     .iter()
        //     .filter_map(|x| match x.2 {
        //         osm_test::PointClassification::InIntersection(p) => Some(p),
        //         _ => None,
        //     })
        //     .collect();
        // out_planet.points.extend(inter);
    });

    //clipping_polygon.clip(&polygon, &mut out_planet);

    // let intersections = polygon.intersections_polygon(&clipping_polygon);
    // intersections
    //     .iter()
    //     .for_each(|(self_arc, other_arc, intersection)| match intersection {
    //         osm_test::PointClassification::InIntersection(point) => {
    //             out_planet.points.push(point.clone())
    //         }
    //         _ => (),
    //     });

    out_planet.to_file(OUT_PLANET_PATH);
}

fn _test_inside_point() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/inside_point.geojson";

    // let planet = Planet::from_osm(PLANET_PATH);
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    planet
        .polygons
        .iter()
        .enumerate()
        // .skip(120)
        // .take(500)
        // .take(1)
        .progress()
        .for_each(|(i, polygon)| {
            // let mut stub = Planet::new();
            // let inside_point = polygon.get_inside_point(&mut stub);
            // if !planet.is_on_land(&inside_point) {
            //     polygon.get_inside_point(&mut out_planet);
            //     out_planet.points.push(inside_point);
            //     polygon
            //         .outline
            //         .windows(2)
            //         .for_each(|v| out_planet.lines.push(osm_test::Arc::new(v[0], v[1])));
            //     //out_planet.polygons.push(polygon.clone());
            //     println!("not inside {}", i);
            // }
        });
    out_planet.to_file(OUT_PLANET_PATH);
}

fn _generate_points() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const LAND_POINTS_PATH: &str = "tests/data/geojson/points_on_land.geojson";
    const WATER_POINTS_PATH: &str = "tests/data/geojson/points_on_water.geojson";
    const N: usize = 4_000_000;

    // let planet = Planet::from_osm(PLANET_PATH);
    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let land_points = Arc::new(Mutex::new(Planet::new()));
    let water_points = Arc::new(Mutex::new(Planet::new()));

    let start = Instant::now();

    let points: Vec<Point> = PointGenerator::new().into_iter().take(N).collect();

    points
        .into_iter()
        .progress()
        .par_bridge()
        .for_each(|point| match planet.inside_is_on_land(&point) {
            true => land_points.lock().unwrap().points.push(point),
            false => water_points.lock().unwrap().points.push(point),
        });

    println!(
        "took {:?} to generate {} points which is {:?} per point",
        start.elapsed(),
        N,
        start.elapsed() / N as u32
    );

    water_points.lock().unwrap().to_file(WATER_POINTS_PATH);
    land_points.lock().unwrap().to_file(LAND_POINTS_PATH);
}
