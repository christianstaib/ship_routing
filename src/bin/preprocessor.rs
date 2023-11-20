use osm_test::geometry::{OsmData, Planet};

use osm_test::spatial_graph::generate_network;

fn main() {
    const osm_file: &str = "tests/data/osm/planet-coastlines.osm.pbf";
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const OUT_PLANET_PATH: &str = "tests/data/test_geojson/network.geojson";
    const NETWORK_PATH: &str = "test_4M.fmi";
    let planet = Planet::from_geojson_file(PLANET_PATH).unwrap();
    // let planet = Planet::from_osm_file(osm_file);

    generate_network(4_000_000, &planet, NETWORK_PATH, OUT_PLANET_PATH);
}
