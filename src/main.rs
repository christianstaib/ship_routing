use std::time::Instant;

use indicatif::ProgressIterator;
use osm_test::{Arc, CollisionDetector, Planet, Point, Polygon, Tiling};

fn main() {
    test_clipping();
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    // const PLANET_PATH: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    // let planet = Planet::from_osm(PLANET_PATH);
    // planet.to_file(OUT_PLANET_PATH);
    let mut out_planet = Planet::new();
    planet
        .polygons
        .sort_by_key(|polygon| -1 * polygon.outline.len() as isize);
    planet.polygons = planet.polygons.into_iter().take(5).collect();
    //out_planet.polygons = planet.polygons.clone();

    let mut quadtree = CollisionDetector::new();

    println!("adding polygons to partition");
    planet
        .polygons
        .iter()
        .cloned()
        .progress()
        .for_each(|polygon| quadtree.add_polygon(polygon));

    // println!("updating midpoints");
    // quadtree.update_midpoints();

    println!("generating points");

    let outlines: Vec<Polygon> = quadtree
        .spatial_partition
        .get_leafes()
        .iter()
        .map(|x| Polygon::new(x.boundary.outline.clone()))
        .collect();

    quadtree
        .spatial_partition
        .get_leafes()
        .iter()
        .for_each(|quad| match &quad.node_type {
            osm_test::NodeType::Internal(_) => panic!(""),
            osm_test::NodeType::Leaf(arc) => {
                let mut arc_copy = arc.clone();
                let old = arc_copy.len();
                arc_copy.dedup();
                assert!(arc.len() == arc_copy.len());
            }
        });

    outlines.iter().for_each(|outline| {
        outline
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| out_planet.lines.push(arc))
    });

    planet.polygons.iter().for_each(|outline| {
        outline
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| out_planet.lines.extend(make_good_line(arc)))
    });

    let start = Instant::now();
    let n = 1_000;
    for _ in (0..n).progress() {
        // let point = Point::from_geodetic(43.9800, -114.0442); //Point::random();
        // let point22 = Point::from_geodetic(19.9431, 96.5227); //Point::random();
        let point = Point::random();
        let point22 = Point::random();
        let ray = Arc::new(&point, &point22);
        let mut true_numb = planet.intersections(&ray);
        let mut my_numb = quadtree.intersections(&ray);
        let old_len_true = true_numb.len();
        let old_len_my = my_numb.len();
        true_numb.sort_by(|x, y| x.lon().total_cmp(&y.lon()));
        my_numb.sort_by(|x, y| x.lon().total_cmp(&y.lon()));
        true_numb.dedup();
        let old_my_copy = my_numb.clone();
        my_numb.dedup();
        if true_numb.len() != old_len_true {
            println!("true num contained dupes: ");
        }
        if my_numb.len() != old_len_my {
            println!(
                "my num contained dupes. old:{}, new:{}",
                old_len_my,
                my_numb.len()
            );
            let removed_elements: Vec<Point> = old_my_copy
                .windows(2) // look at each pair of consecutive elements
                .filter_map(|window| {
                    if window[0] == window[1] {
                        Some(window[0].clone()) // if they are the same, take one of them
                    } else {
                        None // otherwise, none of them was removed
                    }
                })
                .collect();
            out_planet.points.extend(removed_elements);
        }

        //assert!(my_numb.len() == true_numb.len());
        if true_numb.len() != my_numb.len() {
            println!("true:{} my:{}", true_numb.len(), my_numb.len());
            println!("{:?}", true_numb);
            println!("{:?}", my_numb);
            // my_numb.retain(|&p| true_numb.iter().any(|&x| p.equals(&x)));
            out_planet.lines.extend(make_good_line(ray));
            out_planet.points.extend(my_numb.clone());
        }
        // assert_eq!(
        //     planet.intersections(&ray).len(),
        //     quadtree.intersections(&ray)
        // );
        //if quadtree.is_on_polygon(&point) {
        //    out_planet.points.push(point);
        //}
    }
    println!("{:?}", start.elapsed() / n);

    out_planet.to_file(OUT_PLANET_PATH);
}

fn make_good_line(line: Arc) -> Vec<Arc> {
    let mut arcs = vec![line];
    while arcs[0].central_angle() > 0.025 {
        arcs = arcs
            .iter()
            .map(|arc| {
                let middle = arc.middle();
                vec![Arc::new(&arc.from(), &middle), Arc::new(&middle, &arc.to())]
            })
            .flatten()
            .collect();
    }
    arcs.retain(|arc| (arc.from().lon() - arc.to().lon()).abs() < 10.0);
    arcs
}
