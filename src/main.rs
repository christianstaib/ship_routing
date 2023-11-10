use osm_test::generate_network;
use osm_test::Planet;

fn main() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const NETWORK_PATH: &str = "test.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();

    generate_network(&planet, NETWORK_PATH, OUT_PLANET_PATH);
}
