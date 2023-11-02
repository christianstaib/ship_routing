use indicatif::ProgressIterator;
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

fn split_recangle(rectangle: &Polygon) -> Vec<Polygon> {
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

    let np = Point::from_geodetic(90.0, 0.0);
    let sp = Point::from_geodetic(-90.0, 0.0);
    let mid_ring: Vec<f64> = vec![-180.0, -90.0, 0.0, 90.0, 180.0];
    let mid_ring: Vec<Point> = mid_ring
        .iter()
        .map(|&lon| Point::from_geodetic(0.0, lon))
        .collect();
    let upper_ring: Vec<Point> = mid_ring
        .iter()
        .map(|mid| Arc::new(&mid, &np).middle())
        .collect();
    let lower_ring: Vec<Point> = mid_ring
        .iter()
        .map(|mid| Arc::new(&mid, &sp).middle())
        .collect();
    let mid_ring: Vec<f64> = vec![-135.0, -45.0, 45.0, 135.0, -135.0];
    let mid_ring: Vec<Point> = mid_ring
        .iter()
        .map(|&lon| Point::from_geodetic(0.0, lon))
        .collect();

    for i in 0..4 {
        let polygon = Polygon::new(vec![
            upper_ring[i].clone(),
            mid_ring[i].clone(),
            upper_ring[i + 1].clone(),
            np.clone(),
            upper_ring[i].clone(),
        ]);
        out_planet.polygons.push(polygon);

        let polygon = Polygon::new(vec![
            lower_ring[i].clone(),
            sp.clone(),
            lower_ring[i + 1].clone(),
            mid_ring[i].clone(),
            lower_ring[i].clone(),
        ]);
        out_planet.polygons.push(polygon);

        let polygon = Polygon::new(vec![
            mid_ring[i].clone(),
            lower_ring[i + 1].clone(),
            mid_ring[i + 1].clone(),
            upper_ring[i + 1].clone(),
            mid_ring[i].clone(),
        ]);
        out_planet.polygons.push(polygon);
    }
    for _ in 0..0 {
        out_planet.polygons = out_planet
            .polygons
            .iter()
            .map(|polygon| split_recangle(polygon))
            .flatten()
            .collect();
    }

    out_planet.polygons = vec![out_planet.polygons[8].clone()];
    //.retain(|polygon| planet.is_on_polygon(&polygon.inside_point));

    for _ in 0..100000 {
        let point = Point::random();
        if out_planet.is_on_polygon(&point) {
            out_planet.points.push(point);
        }
    }

    out_planet.to_file(OUT_PLANET_PATH);
}
