use osm_test::{Arc, Planet, Point};

#[test]
fn test_point_on_land() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const ON_LAND_PATH: &str = "tests/data/geojson/points_on_land.geojson";

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
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const ON_WATER_PATH: &str = "tests/data/geojson/points_on_water.geojson";

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

#[test]
#[ignore]
fn test_single_point_on_water() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const PLANET_OUT_PATH: &str = "tests/data/test_geojson/single_point.geojson";
    let mut planet = Planet::from_file(PLANET_PATH).unwrap();
    let mut water_point = Point::from_geodetic(62.31948518098084, 154.36065753944712);
    let north_pole = Point::from_geodetic(90.0, 0.0);
    let ray = Arc::new(water_point, north_pole);
    let intersections = planet.intersections(&ray);
    planet.points.extend(intersections.clone());
    let mut old_water_point = water_point.clone();
    while water_point.lat < 89.0 {
        water_point.lat += 0.1;
        let line = Arc::new(old_water_point, water_point);
        planet.lines.push(line);
        old_water_point = water_point;
    }
    planet.to_file(PLANET_OUT_PATH);

    assert!(intersections.len() % 2 == 1);
}
