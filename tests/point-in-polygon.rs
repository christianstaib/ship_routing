use osm_test::{Arc, Planet, Point};

#[test]
fn test_point_on_land() {
    const PLANET_PATH: &str = "data/test_geojson/planet_simplified.geojson";
    const ON_LAND_PATH: &str = "data/test_geojson/true_land.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let on_land = Planet::from_file(ON_LAND_PATH).unwrap();

    on_land.points.iter().for_each(|point| {
        assert!(
            planet.fast_is_on_land(point),
            "point {:?} should be on land but isn't",
            point
        )
    });
}

#[test]
fn test_point_on_water() {
    const PLANET_PATH: &str = "data/test_geojson/planet_simplified.geojson";
    const ON_WATER_PATH: &str = "data/test_geojson/true_water.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let on_water = Planet::from_file(ON_WATER_PATH).unwrap();

    on_water.points.iter().for_each(|point| {
        assert!(
            !planet.fast_is_on_land(point),
            "point {:?} should be on water but isn't",
            point
        )
    });
}

// #[test]
// fn test_single_point_on_water() {
//     const PLANET_PATH: &str = "data/test_geojson/planet_simplified.geojson";
//     const PLANET_OUT_PATH: &str = "data/geojson/planet.geojson";
//     let mut planet = Planet::from_file(PLANET_PATH).unwrap();
//     let mut water_point = Point::from_geodetic(-77.23507365492551, 227.42187500000443);
//     let intersections = planet.fast_intersections(&water_point);
//     planet.points.extend(intersections.clone());
//     let mut old_water_point = water_point.clone();
//     while water_point.lat < 89.0 {
//         water_point.lat += 0.1;
//         let line = Arc::new(old_water_point, water_point);
//         planet.lines.push(line);
//         old_water_point = water_point;
//     }
//     planet.to_file(PLANET_OUT_PATH);
//
//     assert!(planet.fast_is_on_land(&water_point));
// }
