use indicatif::ProgressIterator;
use osm_test::{CollisionDetection, Planet, PlanetGrid};
use rayon::prelude::*;

#[test]
fn test_point_on_land() {
    const PLANET_PATH: &str = "tests/data/geojson/planet.geojson";
    const ON_LAND_PATH: &str = "tests/data/geojson/points_on_land.geojson";

    let planet = Planet::from_file(PLANET_PATH).unwrap();
    let on_land = Planet::from_file(ON_LAND_PATH).unwrap();

    println!("generating grid");
    let mut planet_grid = PlanetGrid::new(500);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));

    println!("updating midpoints");
    planet_grid
        .spatial_partition
        .update_midpoint_with_planet(&planet);

    on_land.points.par_iter().for_each(|point| {
        assert!(
            planet_grid.is_on_polygon(point),
            "point {} should be on land but isn't",
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

    println!("generating grid");
    let mut planet_grid = PlanetGrid::new(500);
    planet
        .polygons
        .iter()
        .progress()
        .for_each(|polygon| planet_grid.add_polygon(polygon));

    println!("updating midpoints");
    planet_grid
        .spatial_partition
        .update_midpoint_with_planet(&planet);

    on_water.points.par_iter().for_each(|point| {
        assert!(
            !planet_grid.is_on_polygon(point),
            "point {} should be on water but isn't",
            point
        )
    });
}
