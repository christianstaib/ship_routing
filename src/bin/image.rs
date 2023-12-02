extern crate image;
extern crate imageproc;

use image::{Rgb, RgbImage};
use imageproc::{drawing::draw_antialiased_line_segment_mut, pixelops::interpolate};
use indicatif::ProgressIterator;
use osm_test::geometry::Planet;

pub fn scale(input: f64, input_min: f64, input_max: f64, output_min: u32, output_max: u32) -> i32 {
    let input_range = input_max - input_min;
    let output_range = output_max as f64 - output_min as f64;
    let scaled_value = ((input - input_min) / input_range) * output_range;
    (scaled_value + output_min as f64).round() as i32
}

fn main() {
    // Create a new Rgba image with a white background
    let red = Rgb([255u8, 0u8, 0u8]);

    // lon, lat
    let min: (f64, f64) = (-180.0, -90.0);
    let max: (f64, f64) = (180.0, 90.0);

    let pix_per_unit = 100.0;
    let x_pix = ((max.0 - min.0) * pix_per_unit) as u32;
    let y_pix = ((max.1 - min.1) * pix_per_unit) as u32;
    let mut image = RgbImage::new(x_pix, y_pix);
    println!("{}x{}", x_pix, y_pix);

    let planet = Planet::from_geojson_file("tests/data/test_geojson/network.geojson").unwrap();
    for arc in planet.arcs.iter().progress() {
        // Define the points
        let start = arc.from().to_geojson_vec();
        let end = arc.to().to_geojson_vec();

        let start = (
            scale(start[0], min.0, max.0, 0, x_pix),
            scale(-start[1], min.1, max.1, 0, y_pix),
        );
        let end = (
            scale(end[0], min.0, max.0, 0, x_pix),
            scale(-end[1], min.1, max.1, 0, y_pix),
        );

        // Draw the line
        draw_antialiased_line_segment_mut(&mut image, start, end, red, interpolate);
    }

    // Save the image
    image.save("output.png").unwrap();

    println!("Image saved as 'output.png'");
}
