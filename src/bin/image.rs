extern crate image;
extern crate imageproc;

use osm_test::geometry::Planet;

fn main() {
    let mut network = Planet::from_geojson_file("tests/data/test_geojson/network.geojson").unwrap();
    let planet = Planet::from_geojson_file("tests/data/test_geojson/planet.geojson").unwrap();

    network
        .arcs
        .extend(planet.polygons.iter().flat_map(|polygon| polygon.arcs()));

    network.to_image("output.png");
}
