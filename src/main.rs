use osm_test::{Arc, Planet, Point, Polygon};

fn main() {
    test_clipping();
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

fn split_recangle(rectangle: &Polygon, planet: &mut Planet) -> Vec<Polygon> {
    assert!(rectangle.outline.len() == 5);
    let arcs: Vec<Arc> = rectangle
        .outline
        .windows(2)
        .map(|arc| Arc::new(&arc[0], &arc[1]))
        .collect();

    let o = rectangle.outline.clone();

    let m: Vec<Point> = arcs.iter().map(|arc| arc.middle()).collect();

    let d0 = Arc::new(&m[0], &m[2]);
    let d1 = Arc::new(&m[1], &m[3]);

    let middle = d0.intersection(&d1).expect("should intersection");
    let p0 = Polygon::new(vec![
        m[3].clone(),
        middle.clone(),
        m[2].clone(),
        o[3].clone(),
        m[3].clone(),
    ]);

    let p1 = Polygon::new(vec![
        middle.clone(),
        m[1].clone(),
        o[2].clone(),
        m[2].clone(),
        middle.clone(),
    ]);

    let p2 = Polygon::new(vec![
        m[0].clone(),
        o[1].clone(),
        m[1].clone(),
        middle.clone(),
        m[0].clone(),
    ]);

    let p3 = Polygon::new(vec![
        o[0].clone(),
        m[0].clone(),
        middle.clone(),
        m[3].clone(),
        o[0].clone(),
    ]);
    vec![p0, p1, p2, p3]
}

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    let sw = Point::from_geodetic(50.0, 50.0);
    let ne = Point::from_geodetic(65.0, 60.0);
    let rectangle = generate_rectangle(&sw, &ne);
    // let rectangle = Polygon::new(vec![
    //     Point::from_geodetic(4.251634083025594, -5.0),
    //     Point::from_geodetic(4.263471412056005, -2.2518832583820663),
    //     Point::from_geodetic(6.02062449764739, -2.2563497090195175),
    //     Point::from_geodetic(6.0, -5.0),
    //     Point::from_geodetic(4.251634083025594, -5.0),
    // ]);
    // out_planet.points.push(rectangle.inside_point);

    let mut sub_rectangles = vec![rectangle];
    for _ in 0..6 {
        sub_rectangles = sub_rectangles
            .iter()
            .map(|rectangle| split_recangle(rectangle, &mut out_planet))
            .flatten()
            .collect();
        println!("len is {}", sub_rectangles.len());
    }

    out_planet.polygons.extend(sub_rectangles);

    //out_planet.polygons.push(rectangle);

    out_planet.to_file(OUT_PLANET_PATH);
}
