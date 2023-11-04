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
    let n = 100_000;
    for _ in (0..n).progress() {
        let point1 = Point::random();
        let point2 = Point::random();
        let ray = Arc::new(&point1, &point2);
        let true_numb = planet.intersections(&ray);
        let my_numb = quadtree.intersections(&ray);

        assert!(my_numb.len() == true_numb.len());

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
    while arcs[0].central_angle() > 0.005 {
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
