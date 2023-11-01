use osm_test::{Planet, Point, Polygon};

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

fn test_clipping() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/grid.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut out_planet = Planet::new();

    let sw = Point::from_geodetic(-5.0, -5.0);
    let ne = Point::from_geodetic(5.0, 5.0);
    let rectangle = generate_rectangle(&sw, &ne);
    out_planet.polygons.push(rectangle);

    out_planet.to_file(OUT_PLANET_PATH);
}
