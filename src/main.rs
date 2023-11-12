use indicatif::ProgressIterator;
use osm_test::fmi::Fmi;
use osm_test::generate_network;
use osm_test::read_paths;
use osm_test::Linestring;
use osm_test::Planet;

fn main() {
    //translate_route();
    _generate();
}

fn _translate_route() {
    let size = "400k";
    let fmi = Fmi::new(format!("test_{}.fmi", size).as_str());
    println!("read fmi");
    let paths = read_paths(format!("route_{}.csv", size).as_str(), &fmi);
    println!("read planet");
    let mut planet = Planet::new();
    planet.linestrings.extend(
        paths
            .iter()
            .progress()
            .map(|path| Linestring::new(path.clone())),
    );
    planet.to_geojson_file(format!("route_{}.geojson", size).as_str());
}

fn _generate() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const NETWORK_PATH: &str = "test_400k.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();

    generate_network(400_000, &planet, NETWORK_PATH, OUT_PLANET_PATH);
}
