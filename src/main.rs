use osm_test::fmi::Fmi;
use osm_test::generate_network;
use osm_test::read_paths;
use osm_test::Linestring;
use osm_test::Planet;

fn main() {
    let fmi = Fmi::new("test.fmi");
    let paths = read_paths("route.csv", &fmi);
    let mut planet = Planet::new();
    planet
        .linestrings
        .extend(paths.iter().map(|path| Linestring::new(path.clone())));
    planet.to_geojson_file("route.geojson");
}

fn generate() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const NETWORK_PATH: &str = "test.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();

    generate_network(&planet, NETWORK_PATH, OUT_PLANET_PATH);
}
